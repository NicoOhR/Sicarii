# Code Generation

With the context of the architecture of the virtual machine architecture, we're 
as ready as we'll ever be to tackle code generation. Code generation is generally the 
last stop of the compilation process, where we go from the last level of the 
intermediate representation to the byte code which is executed by the target.

The `Generator` class holds the program, the bytecode being built, and two
symbol tables — one for globals and one mapping local `SymbolId`s to their
frame slot index:

```cpp
// generator.hpp
class Generator {
  const core::Program &program;
  std::vector<core::SymbolId> global_symbols;
  std::map<core::SymbolId, size_t> local_symbols;
  std::vector<ISA::Instruction> bytecode;
  // ...
};
```

We need to keep the little bit of state so that so that we can discern whether we need to load a global 
or local symbol. This could have just easily been handled at the scoping step, annotating each `SymbolID`
as either local or global.

`generate()` iterates over every top-level item and appends a `HALT` at the end:

```cpp
// generator.cpp
std::vector<ISA::Instruction> Generator::generate() {
  for (auto &top : this->program)
    emit_top(top);
  add_instruction(ISA::Operation::HALT, std::nullopt);
  return this->bytecode;
}
```

each `emit_` appends instructiosn before dispatching to the next `emit_` function that needs to run, which results in the following tree 

```text
  generate()
  └── emit_top(Top)
      ├── [Define] → emit_top_define(Define)
      │   └── emit_expr(rhs) → [see emit_expr]
      │       └── MKGLOBAL
      └── [Expr]  → emit_expr(Expr)

  emit_expr(Expr)
  ├── [Apply]  → emit_apply(Apply)
  │   ├── callee is Var (builtin)  → emit_expr*(args) → <builtin opcode>
  │   ├── callee is Var (non-builtin) → emit_var(Var) → emit_expr*(args) → CALL
  │   ├── callee is Lambda → emit_lambda(Lambda) → emit_expr*(args) → CALL
  │   ├── callee is Apply  → emit_apply(Apply)   → emit_expr*(args) → CALL
  │   └── callee is other  → emit_expr(callee)   → emit_expr*(args) → CALL
  ├── [Lambda] → emit_lambda(Lambda)
  │   ├── JMP (→ MKCLOSURE)
  │   ├── ENTER n
  │   ├── emit_expr*(body[0..n-2]) + DROP each
  │   ├── emit_expr(body.back())
  │   ├── NROT n+1 / DROP n / RET
  │   └── MKCLOSURE enter_offset
  ├── [Const]  → emit_const(Const)
  │   └── PUSH value
  ├── [Cond]   → emit_cond(Cond)
  │   ├── emit_expr(condition)
  │   ├── CJMP (→ then)
  │   ├── emit_expr(otherwise)
  │   ├── JMP  (→ after then)
  │   └── emit_expr(then)
  ├── [Var]    → emit_var(Var)
  │   ├── const_builtin   → <const opcode>
  │   ├── global_symbols  → LOADGLOBAL id
  │   └── local_symbols   → GETLOCAL  idx
  ├── [Set]    → emit_set(Set)
  │   ├── emit_expr(rhs)
  │   ├── local_symbols   → SETLOCAL idx
  │   └── global_symbols  → MUTGLOBAL id
  └── [Undef]  → PUSH 0
```

We've alreayd covered the calling convention in the VM article, so we'll explain the rest of the produced code in this article

## Constants and Variables

Constants are the simplest case, a literal value becomes a single `PUSH`:

```cpp
// generator.cpp
void Generator::emit_const(const core::Const &const_var) {
  add_instruction(ISA::Operation::PUSH, const_var.value);
}
```

Variables require a lookup, where we check the global and local symbols, emitting the corrosponding op code

```cpp
// generator.cpp
void Generator::emit_var(const core::Var &variable) {
  if (auto it = this->const_builtins.find(variable.id);
      it != this->const_builtins.end()) {
    add_instruction(it->second, std::nullopt); // e.g. PUSHNIL
    return;
  }
  if (std::find(global_symbols.begin(), global_symbols.end(), variable.id) !=
      global_symbols.end()) {
    add_instruction(ISA::Operation::LOADGLOBAL, variable.id);
  } else {
    add_instruction(ISA::Operation::GETLOCAL, local_symbols[variable.id]);
  }
}
```

## Globals and Mutation

A top-level `define` registers the symbol in `global_symbols` *before* emitting
the right-hand side, which allows recursive definitions to resolve the name:

```cpp
// generator.cpp
void Generator::emit_top_define(const core::Define &def) {
  this->global_symbols.push_back(def.name);
  Generator::emit_expr(*def.rhs);
  add_instruction(ISA::Operation::MKGLOBAL, def.name);
}
```

`set!` emits the new value first, then either `SETLOCAL` or `MUTGLOBAL`
depending on where the binding lives:

```cpp
// generator.cpp
void Generator::emit_set(const core::Set &set_op) {
  emit_expr(*set_op.rhs);
  if (local_symbols.find(set_op.name) != local_symbols.end())
    add_instruction(ISA::Operation::SETLOCAL, local_symbols.at(set_op.name));
  else
    add_instruction(ISA::Operation::MUTGLOBAL, set_op.name);
}
```

## Conditionals

`emit_cond` uses the backpatch pattern: it emits the condition, reserves a
`CJMP` slot, emits the `otherwise` branch, reserves a `JMP` slot, then fills in
both jump targets once it knows the sizes of each branch:

```cpp
// generator.cpp
void Generator::emit_cond(const core::Cond &cond) {
  emit_expr(*cond.condition);
  size_t cjmp_idx = this->bytecode.size();
  add_instruction(ISA::Operation::CJMP, std::nullopt); // target TBD
  emit_expr(*cond.otherwise);
  size_t jmp_idx = this->bytecode.size();
  add_instruction(ISA::Operation::JMP, std::nullopt); // target TBD
  this->bytecode[cjmp_idx].operand = this->bytecode.size() * 9;
  emit_expr(*cond.then);
  this->bytecode[jmp_idx].operand = this->bytecode.size() * 9;
}
```

`CJMP` jumps if the top of the stack is 1, so the `otherwise` branch sits
immediately after it.

## Lambdas and Closures

Lambdas are the most involved case, and follow the the VMs calling convention, which we explained
in detail

```cpp
// generator.cpp
void Generator::emit_lambda(const core::Lambda &lambda) {
  auto saved_locals = this->local_symbols;
  auto n = lambda.formals.size() + saved_locals.size();

  for (size_t i = 0; i < lambda.formals.size(); i++)
    this->local_symbols[*lambda.formals.at(i)] = i;

  // remap captured outer locals to sit after the new formals
  for (auto &[sym_id, outer_idx] : saved_locals)
    this->local_symbols[sym_id] = lambda.formals.size() + outer_idx;

  const auto jmp_idx = this->bytecode.size();
  add_instruction(ISA::Operation::JMP, std::nullopt);
  const auto enter_offset = this->bytecode.size() * 9;
  add_instruction(ISA::Operation::ENTER, n);

  for (size_t i = 0; i < lambda.body.size() - 1; i++) {
    emit_expr(*lambda.body.at(i));
    add_instruction(ISA::Operation::DROP, 1);
  }
  emit_expr(*lambda.body.back());
  add_instruction(ISA::Operation::NROT, n + 1);
  add_instruction(ISA::Operation::DROP, n);
  add_instruction(ISA::Operation::RET, std::nullopt);

  const auto mk_offset = this->bytecode.size();
  add_instruction(ISA::Operation::MKCLOSURE, enter_offset);
  this->bytecode[jmp_idx].operand = mk_offset * 9;

  this->local_symbols = saved_locals;
}
```

## Function Application

Built-in functions, arithmetic, comparisons, list operations are mapped
directly to opcodes in a table keyed by `SymbolId`. When the callee resolves
to one of them the generator emits the arguments then the opcode, with no
`CALL`:

```cpp
// generator.cpp
void Generator::emit_apply(const core::Apply &application) {
  if (auto *var = std::get_if<core::Var>(&application.callee->node)) {
    auto it = this->builtins.find(var->id);
    if (it != this->builtins.end()) {
      for (auto &&arg : application.args)
        emit_expr(*arg);
      add_instruction(it->second, std::nullopt);
      return;
    }
    emit_var(*var); // push closure handle
  } else {
    emit_expr(*application.callee);
  }
  for (auto &&arg : application.args)
    emit_expr(*arg);
  add_instruction(ISA::Operation::CALL, application.args.size());
}
```

For a user-defined function the closure handle is pushed first, then each
argument, then `CALL` with the argument count as its operand.
