# Addenda

In between part one and part two of this write up, the AST undergoes a few transformation;
these transformations are not, in the most platonic sense, strictly necessary for a compiler. 
It is perfectly plausible to jump from surface AST to machine code, it is also, as we say in the biz,
very hard. One of the compiler design tips which I think I internalized the most through this project is that 
compilation is first and foremost a series of string manipulation passes that allow us to reason about a program
in increasingly more primitive ways$$$I've personally taken to calling this programmatic deterritorialization,
a term which I am sure will impress all and any compiler engineer who moonlights as a postmodern philosopher$$$. In pursuit 
of a more pure core language to compile, we introduce the following transformations: *Special forms* are primitives which we privilege
early so that we can break them down into constituent parts later, *desugaring* is the process of breaking up some of those special forms,
*scoping* resolves human named variables to unique numeric variables, *lowering* introduces the *core* language which massively simplifies the 
code generation process.

## Special Forms

The set of special forms we support is fixed at compile time and lives in an
enum:

```cpp
// ast.hpp
enum Keyword { if_expr, let, letrec, lambda, define, set, null };
```

The parser maps the corresponding source strings to those enum values with a
lookup table:

```cpp
// parser.hpp
const std::unordered_map<std::string, Keyword> kwords = {
    {"if", Keyword::if_expr},    {"let", Keyword::let},
    {"lambda", Keyword::lambda}, {"define", Keyword::define},
    {"letrec", Keyword::letrec}, {"set!", Keyword::set},
    {"nil", Keyword::null}};
```

When the lexer produces an atom token the parser first tries to parse it as a
number, then a boolean literal, then checks the keyword table. If a keyword is
found the symbol carries a `Keyword` tag instead of a raw string.

After the initial S-expression tree is built a second pass called
`resolve_forms` walks it and handles each special form:

```cpp
// parser.cpp
void Parser::resolve_forms(SExp &sexp, bool is_top_level) {
  std::visit(
      [this, &sexp, is_top_level](auto &p) {
        // ...
        if constexpr (std::is_same_v<T, List>) {
          if (auto *kw = std::get_if<ast::Keyword>(&sym->value)) {
            switch (*kw) {
            case (ast::Keyword::define): {
              if (!is_top_level)
                throw std::invalid_argument(
                    "Define is only allowed at the top level");
              sexp.node = create_define(p);
              // ...
            }
            case (ast::Keyword::lambda): {
              sexp.node = create_lambda(p);
              // ...
            }
            case (ast::Keyword::let): {
              sexp = create_let(p);
              resolve_forms(sexp, false);
              return;
            }
            case (ast::Keyword::letrec): {
              sexp = create_letrec(p);
              resolve_forms(sexp, false);
            }
              // ...
            }
          }
        }
      },
      sexp.node);
}
```

`define` and `lambda` are validated and left as-is in the tree. `let` and
`letrec` are desugared, rewritten into equivalent combinations of `lambda` and
`set!`.$$$
The term "desugaring" comes from referring to certain language features as "syntactic sugar", these are
parts of the language which do not contribute to the overall expressive power of
the language, but make it easier to program in the language
$$$ The goal, at least at the language design level, is to get the language down to
minimal possible representation so code generation is as simple and efficient as
possible.

## Desugaring

### Let
**`let`** introduces a set of local bindings that are all evaluated before any
name comes into scope. This is equivalently the behavior of a lambda which is
invoked immediatly:

```text
(let ((x e1) (y e2)) body) ->  ((lambda (x y) body) e1 e2)
```

In the above `x` and `y`, which are set to `e1` and `e2` respectively, are
accessible only to the body of this `let` scope, it is hopefully clear how this
is the same as passing `e1` and `e2` to a lambda which has `x` and `y` as
parameters. In the above, the outer expressions, `e1` and `e2` "do not know"
that the program is using them as variables `x` and `y`, in that way, they have
no ability to refer to themselves in their own definitions, which is an important 
restriction of `let` which the above desugar gives us for free. `let` does not
allow for recursion, but clearly encodes the scope and availability of
a variable $$$The stronger compilation time knowledge that `let` affords us
also enables certain optimizations which we will not be discussing in this
article, but are worth looking into$$$

### Let Recursive

`letrec` allows us to use recursion, it's desguaring rules:


```text 
(letrec ((f e)) body)
  ->  (let ((f nil)) (set! f e) body)
  ->  ((lambda (f) (set! f e) body) nil)
```

By adding the second step, `f` is a name that is available to `e` while it's
being evaluated, enabling mutual recursion.

`letrec` therefore reduces to `let` plus mutation, and `let` reduces to
`lambda`. After `resolve_forms` the AST contains only `define`, `lambda`, `if`,
and `set!` as special forms.

## Implementations

```cpp
// parser.cpp — let
SExp Parser::create_let(List &list) {
  // collect the binding names into a formals list ...
  // collect the initialiser expressions separately ...
  List lambda_list;
  lambda_list.list.push_back(make_sexp(Symbol{Keyword::lambda}));
  lambda_list.list.push_back(make_sexp(std::move(vars))); // (x y)
  for (size_t i = 2; i < list.list.size(); i++)
    lambda_list.list.push_back(std::move(list.list.at(i))); // body
  desugared.list.push_back(make_sexp(std::move(lambda_list)));
  for (auto &value : values_list)
    desugared.list.push_back(std::move(value)); // e1 e2
  return SExp{.node = std::move(desugared)};
}
```

```cpp
// parser.cpp — letrec
SExp Parser::create_letrec(List &list) {
  // build (name nil) pairs for the let bindings ...
  // build (set! name definition) expressions ...
  desugared.list.push_back(make_sexp(Symbol{Keyword::let}));
  desugared.list.push_back(make_sexp(std::move(undefined_names)));
  for (auto &set_name : set_names.list)
    desugared.list.push_back(std::move(set_name));
  for (size_t i = 2; i < list.list.size(); i++)
    desugared.list.push_back(std::move(list.list.at(i)));
  return create_let(desugared); // reduce to let, then to lambda
}
```

## Scoping

This Lisp uses _lexical scoping_: the meaning of a name is determined by where it
appears in the source text, not where a function happens to be called from.
Concretely, a `lambda` introduces a new scope that can see everything in the
surrounding scope. A symbol should then be assigned to the definition as local
as possible. We solve this problem by representing our scopes as tables which
relate to each other with tree semantics:

```cpp
// scoper.hpp
struct SymbolTable {
  size_t scope_id;
  std::unordered_map<std::string, Binding> symbols;
  std::vector<std::unique_ptr<SymbolTable>> children;
  SymbolTable *parent;
};
```

The root table is the global scope, pre-populated with the built-in functions
(`+`, `-`, `cons`, `car`, `cdr`, …). Every `lambda` in the source gets its own
child table. `parent` pointers let us walk back up to resolve names that were
not declared locally.

The `Scoper` runs in two phases: `run` builds the tree and assigns each binding
a unique integer ID; `resolve` replaces every string identifier in the AST with
that integer.

### Building the Scope Tree

```cpp
// scoper.cpp
case (ast::Keyword::lambda): {
  // collect the formal parameters, assigning each a fresh ID
  std::unordered_map<std::string, Binding> syms;
  for (auto &arg : args->list) {
    if (auto *ident = std::get_if<std::string>(&arg->value)) {
      syms[*ident] = Binding{.kind = BindingKind::VALUE,
                             .value = scoper->next_binding_id++};
    }
  }
  // create a child scope and attach it to the parent
  auto child_scope = std::make_unique<SymbolTable>();
  child_scope->scope_id = ++curr_idx;
  child_scope->symbols = std::move(syms);
  child_scope->parent = parent;
  sexp.scope_id = curr_idx; // tag the SExp with its scope
  SymbolTable *child_ptr = child_scope.get();
  parent->children.push_back(std::move(child_scope));
  // recurse into the lambda body under the new scope
  for (auto &child : node.list)
    self(*child, child_ptr);
  return;
}
```

### Resolving Names

```cpp
// scoper.cpp
if constexpr (std::is_same_v<T, ast::Symbol>) {
  if (auto *ident = std::get_if<std::string>(&node.value)) {
    auto binding = scoper->search(*ident, curr_scope);
    node.value = ast::SymbolID{binding.value};
  }
}
```

`search` walks up the parent chain until it finds the name or runs out of
scopes:

```cpp
// scoper.cpp
Binding Scoper::search(std::string ident, size_t lowest_scope) {
  auto table = Scoper::find_table(lowest_scope);
  while (table != nullptr) {
    if (auto it = table->symbols.find(ident); it != table->symbols.end())
      return it->second;
    else
      table = table->parent;
  }
  throw std::invalid_argument(ident + " symbol not found in any scope");
}
```

After this pass every `std::string` symbol in the AST has been replaced by a
`SymbolID` which is guaranteed to be unique, two references to the same binding share the same integer 
and the scope tree can be discarded.

## Code Lowering and the Core Language

The AST the parser produces is still quite "surface-y", it carries all of the
syntactic forms and still uses the `SExp` / `List` / `Symbol` vocabulary. The
_core language_ is a separate, typed IR that strips that away and gives each
concept its own concrete type:

```cpp
// core.hpp
struct Const {
  uint64_t value;
};
struct Var {
  SymbolId id;
};
struct Apply {
  std::unique_ptr<Expr> callee;
  std::vector<std::unique_ptr<Expr>> args;
};
struct Lambda {
  std::vector<std::unique_ptr<SymbolId>> formals;
  std::vector<std::unique_ptr<Expr>> body;
};
struct Cond {
  std::unique_ptr<Expr> condition;
  std::unique_ptr<Expr> then;
  std::unique_ptr<Expr> otherwise;
};
struct Define {
  SymbolId name;
  std::unique_ptr<Expr> rhs;
};
struct Set {
  SymbolId name;
  std::unique_ptr<Expr> rhs;
};

struct Expr {
  std::variant<Apply, Define, Lambda, Const, Cond, Var, Set, Undef> node;
};
```

Because we already desugared `let`/`letrec` into lambdas while resolving those special forms, and resolved
all names to integers, the lowering pass is mostly pattern matching. The
`Lowerer::lower_expr` function dispatches on the shape of the AST node:

```cpp
// core.cpp
core::Expr core::Lowerer::lower_expr(const ast::SExp &sexp) {
  if (const auto lst = std::get_if<ast::List>(&sexp.node)) {
    if (const auto kw = std::get_if<ast::Keyword>(&sym->value)) {
      switch (*kw) {
      case (ast::Keyword::if_expr):
        ret.node = lower_condition(sexp);
        return ret;
      case (ast::Keyword::lambda):
        ret.node = lower_lambda(sexp);
        return ret;
      case (ast::Keyword::set):
        ret.node = lower_set(sexp);
        return ret;
      case (ast::Keyword::define):
        throw std::invalid_argument("Define is only allowed at the top level");
      default:
        break;
      }
    }
    ret.node = lower_apply(sexp); // anything else is a function call
    return ret;
  }
  // atoms: numbers/bools -> Const, undef -> Undef, SymbolID -> Var
  if (const auto sym = std::get_if<ast::Symbol>(&sexp.node)) {
    if (std::get_if<uint64_t>(&sym->value) || std::get_if<bool>(&sym->value))
      ret.node = lower_const(sexp);
    else if (std::get_if<ast::Undef>(&sym->value))
      ret.node = lower_undef(sexp);
    else
      ret.node = lower_var(sexp); // SymbolID -> Var
  }
  return ret;
}
```

For `lambda`, the formals list and body expressions are extracted by position —
the structure is guaranteed by the parser so there is no additional validation:

```cpp
// core.cpp
core::Lambda core::Lowerer::lower_lambda(const ast::SExp &sexp) {
  // List(Keyword(Lambda) List(args) SExp(body)*)
  core::Lambda ret;
  if (const auto lst = std::get_if<ast::List>(&sexp.node)) {
    if (const auto args = std::get_if<ast::List>(&lst->list.at(1)->node)) {
      for (size_t i = 0; i < args->list.size(); i++) {
        if (const auto id = std::get_if<ast::SymbolID>(&sym->value))
          ret.formals.push_back(std::make_unique<core::SymbolId>(id->id));
      }
      for (size_t i = 2; i < lst->list.size(); i++)
        ret.body.push_back(std::make_unique<core::Expr>(
            core::Lowerer::lower_expr(*lst->list.at(i))));
    }
  }
  return ret;
}
```

`define` is only permitted at the top level (the `lower_top` entry point handles
it separately from `lower_expr`), which means the core `Program` is a flat list
of top-level definitions and expressions — nothing nested can introduce a new
global binding.

```cpp
// core.hpp
using Top = std::variant<Define, Expr>;
using Program = std::vector<Top>;
```

The core language is what the code generator consumes. 

