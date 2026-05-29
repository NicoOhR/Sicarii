
# The Stack

The exact phase boundary between an interpreted and compiled
language is slightly fuzzy; Java, generally thought of as compiled,
targets a virtual machine after several layers of IR, Python, generally
thought of as interpreted... Targets a virtual machine after several layers
of IR? What's the difference?

I attribute the fact that a good bunch of languages are a little bit of both
is that a purely interpreted language, one
which takes your mushy-human-programmer code and immediately executes it
on a compatible abstract machine, will usually be a very unpleasant
programming experience. Turns out, compilation, or at least intermediate
representation of source code, is *very useful*. Compilation gives the
language designer the opportunity to do optimizations, check for
correctness, and allow the programmer to use more intricate program
organization, think classes and modules. However, compiling all the way
down to something that talks to your hardware is not always feasible or
even desirable. There are, and I am sorry to tell you this, a lot of
different kinds of hardware, and not every language designer wants to
spend the time and effort on designing the compiler backend for the sake
of portability. Less to the point but still an important motivation for
VMs in language design, it is relatively common for a language to be
"embedded" in another system, notable examples include Lua, eBPF in
the linux kernel, and maybe something like GDScript in Godot. If a domain
specific language like this is a good idea or not is a question for the
culture that I will not answer.


## The Abstract Machine

A stack is an abstract data structure following LIFO semantics, that is,
a piece of data enters, and does not leave until all entries added after
it have been removed from the stack. *The Stack* is the name I took to
calling the virtual machine that I was targeting for the purposes of this
part of the project. Before I describe the full architecture I will do
a little bit of apologia for the design. When I started the project I wanted most of the engineering to 
be "from first principles", as such, this was probably the most naive
part of the project, going off of no real specification, and only vaguely
referencing sources like clox from crafting interpreters. This is also why this is not just an SECD machine, though I might 
implement it as a separate target later. When I started
the project I had in mind a pure Forth-like stack machine, without heaps
or registers and a relatively limited ISA. As it turns out, and I should
have foreseen this from the onset frankly, it's impossible to implement
cool things like closures without dynamic data structures, for reasons
that I will explain when we get there. Even giving up a completely static
memory model only gets us so far before we need to contend with other
difficult bits and pieces of a perfect stack model, namely that it becomes
really ugly if a function uses variables in any order that is not precisely
LIFO. The generator would need to pop out all of the variables above the variable of interest, pull that one to the top, and push them all back in. So when we say a "stack" vm, we generally mean a structure which
enforces the stack semantics *for code environments* rather than for
*individual variables*. With that little preamble out of the way we can
finally get started on sketching out the virtual machine that we will be generating code for.

The virtual machine is built of the following components 
* A memory primitive, which the rest of the VM interacts with *only through shared pointers*, named `Cell`, imagine this as a physical memory location
* `program_mem` which stores our program instructions.
* A program counter register, `pc`, which points to where in `program_mem` the program currently is
* A data "stack", which is a vector of shared pointers to `Cells`
* A return stack, which is an actual stack of `Cells`
* A `frame_base` which points to where in the data stack our local scope starts, and a `frame_base_stack` which tracks where we should restore the base to after returning from a local scope
* A `heap` which stores heap objects, that is, code environments and list primitives
* `global_tbl` which is a map of global variables and "where they live", this is in itself a form of dynamic memory similar to heap

This is, largely, isomorphic to an SECD machine$$$the original SECD machine did not support global variables, but later extensions did. If you do not know what SECD machine is, it's not necessary to understanding the rest of the article, but is a very interesting piece of programming history$$$ but the general philosophy and code generation strategy are different enough as to be worth treating this machine as a separate entity.



## The Memory Model

The `Cell` and `HeapObject` types underpin most of the operations of the VM, so it's worth looking at them in greater detail

```C++
struct Cell {
  int64_t value;
  bool function = false;
  bool pair = false;
  bool null = false;
};

struct CodeEnv {
  uint64_t code_idx;
  std::vector<std::shared_ptr<Cell>> captured_vars;
};

struct Pair {
  std::shared_ptr<Cell> head;
  std::shared_ptr<Cell> tail;
};

using HeapObject = std::variant<Pair, CodeEnv>;
```

`Cell` is an almost self explanatory single memory unit, it contains one signed, 64 bit integer value, a function pair and null flag$$$ If, and this is not out of the question, I end up designing something in verilog to model this system, I will probably end up cutting into the most significant bits of the integer to represent the flags$$$. The `CodeEnv` struct represents the code environment that a lambda runs in, it's necessary for this to be a heap structure, that is, to have a dynamic lifetime, because the lambda could be called past the lifetime of the variables that it originally lived in. For that reason, closures/closure environments generally require heap allocation, Rust being a notable exception (as it typically is), which gets around the problem of a closure using a dead variable by way of its ownership model.

`Pair` is just a cons cell, this object allows us to support lists and list programming in the virtual machine. The `Pair` struct has to be a heap object for a similar reason to the `CodeEnv` object, namely, we can't actually say at comp time how large a tree represented by the cons cells taken together will be.

# The ISA

With that, we're ready to look at the ISA that the program uses to interact with the virtual machine. A lot of these are likely familiar, so we'll be skipping the arithmetic, logic, and transfer opcodes to focus on the control and list instructions in the next section, where we detail how variables are handled, and then the calling convention$$$ I am certain that I will eventually, probably after this particular write up, get around to writing  more cohesive documentation $$$.

## Arithmetic Operations

| Opcode | Operand | Stack In | Stack Out |
|--------|---------|----------|-----------|
| `add`  | —       | 2        | 1         |
| `sub`  | —       | 2        | 1         |
| `mul`  | —       | 2        | 1         |
| `div`  | —       | 2        | 1         |
| `mod`  | —       | 2        | 1         |
| `inc`  | —       | 1        | 1         |
| `dec`  | —       | 1        | 1         |
| `max`  | —       | 2        | 1         |
| `min`  | —       | 2        | 1         |

## Logic Operations

| Opcode | Operand | Stack In | Stack Out |
|--------|---------|----------|-----------|
| `lt`   | —       | 2        | 1         |
| `le`   | —       | 2        | 1         |
| `eq`   | —       | 2        | 1         |
| `ge`   | —       | 2        | 1         |
| `gt`   | —       | 2        | 1         |

## Transfer Operations

| Opcode  | Operand | Stack In | Stack Out |
|---------|---------|----------|-----------|
| `drop`  | u64     | 1        | 0         |
| `dup`   | —       | 1        | 2         |
| `swap`  | —       | 2        | 2         |
| `nrot`  | u64     | 0        | 0         |
| `push`  | u64     | 0        | 1         |

## Control Operations

| Opcode        | Operand | Stack In | Stack Out |
|---------------|---------|----------|-----------|
| `call`        | u64     | 0        | 0         |
| `ret`         | —       | 0        | 0         |
| `jmp`         | addr    | 0        | 0         |
| `cjmp`        | addr    | 1        | 0         |
| `wait`        | —       | 0        | 0         |
| `halt`        | —       | 0        | 0         |
| `mkclosure`   | u64     | 0        | 0         |
| `mkglobal`    | u64     | 1        | 0         |
| `loadglobal`  | u64     | 0        | 0         |
| `mutglobal`   | u64     | 0        | 0         |
| `enter`       | u64     | 0        | 0         |
| `get_local`   | u64     | 0        | 1         |
| `set_local`   | u64     | 1        | 0         |

## List Operations

| Opcode    | Operand | Stack In | Stack Out |
|-----------|---------|----------|-----------|
| `cons`    | —       | 2        | 1         |
| `car`     | —       | 1        | 1         |
| `cdr`     | —       | 1        | 1         |
| `pushnil` | —       | 0        | 1         |
| `isnull`  | —       | 1        | 1         |

# Programming the Machine

## Variables

First we should understand how variables work. We can take it as a promise from the language front end that all variables have a globally unique name, which we'll call `SymbolID`. A variable can be either global or local, let's begin with the simpler of the two and inspect how we interact with locally scoped variables:

* `GETLOCAL n` loads the `n`th variable from the `frame_base` up to the top of the stack. On an implementation level, this pushes a copy of the pointer at `data_stack[frame_base + n]` up to the top of the stack.

* `SETLOCAL n` pop a `Cell` pointer from the top of the stack and then replaces the dereferenced `Cell` found at `data_stack[frame_base + n]`. This propagates to all other shared pointers which pointed at that `Cell` previously.

Local variables are kind of automatic in the way that if it's on the stack above the current frame index, it's accessible as a local variable. Global variables are a bit more tricky as we need to indicate that this global variable should be accessible from everywhere, so we need to add a way to create a new global variable.

* `MKGLOBAL n` creates a new entry in the `global_tbl` with the key `n`, pointing to a shared pointer to the `Cell` which holds the value we want to be global.
* `LOADGLOBAL n` pushes to the stack the variable in the `n` key of the `global_tbl`
* `MUTGLOBAL n`  changes the underlying reference of the `n` key of the `global_tbl` to the top of the stack.

Since we ultimately are the ones who control the code generation, and we're pretty ok that `SETLOCAL` can interact with a global variable unwittingly.

## The Calling Convention

In programming language design, the *calling convention* is ~~the scheme~~ the contract for how functions receive and interact with their parameters, and how functions are expected to provide their outputs. This is almost always considered to be part of the ABI,abstract binary interface, and determines how subroutines and programs are expected to interact with one another.  I will save you the details of the three or four revisions of this calling convention.

First, we should internalize what we mean by code environments being subject to stack semantics rather than the variables. When we `CALL` a closure, which is all functions in this language, we expect it's local variables to be the arguments passed to it by the callee, and the variables that it references in its body, formals and captured variables respectively. To achieve this, we should tell the VM where those start, which we do by setting `frame_base`, and allowing the closure to interact with it's local variables *only in reference to that frame base*. 

But when that called closure has concluded, and the result lies on top of the stack, the callee, which might be a closure in and of it self, still may need to do some work, so we have to restore the frame base to what it was prior to the call, which we achieve through the `frame_base_stack`! Diagramatically, we have the following procedure. The *caller*, just before `CALL n` is invoked:

```text
+-----------------+
|      argn       |
+-----------------+
|       ...       |
+-----------------+
|      arg0       |
+-----------------+
|       c0        | <- Cell containing CodeEnv
+-----------------+
|       ...       | 
+-----------------+
|      argX       | <- frame_base
+-----------------+
|     Bottom      |
+-----------------+
```

`CALL` consumes `c0`, pushes the captured variables, saves `frame_base` to `frame_base_stack`, then moves `frame_base` up to `arg0` — entering the *callee*:

```text
+-----------------+
|      capm       |
+-----------------+
|       ...       |
+-----------------+
|      cap0       |
+-----------------+
|      argn       |
+-----------------+
|       ...       |
+-----------------+
|      arg0       | <- frame_base  (new)
+-----------------+
|       ...       |
+-----------------+
|      argX       | <- frame_base  (saved)
+-----------------+
|     Bottom      |
+-----------------+
```

The closure does what work it needs to, accessing the arguments and captures through `GETLOCAL`. After `NROT n+1` and `DROP n` clean the frame slots, `RET` pops `frame_base_stack`, restoring the caller's view with only the result on top:

```text
+-----------------+
|     result      |
+-----------------+
|       ...       | <- frame_base  (restored)
+-----------------+
|     Bottom      |
+-----------------+
```

Let's now walk through what a function call looks like at the assembly level. Starting with the declaration of a function at comp time:

```stack isa
[0]     JMP  [M+3]          ; skip over the body 

[1]     ENTER  n            ; n = #formals + #captured-outer-locals

<body>                      

[M]     NROT  n+1           ; Function clean up
                           
[M+1]   DROP  n             ; discard the n frame slots, intermediate results of the procedure
                            ; result is now on top for the caller

[M+2]   RET                 ; pop return address, restore old frame_base

[M+3]   MKCLOSURE [1]       ; heap-allocate a CodeEnv:
                            ;   .captured_vars = snapshot of data_stack[frame_base..top)
```

`JMP` simply sets the program counter to the operand, in the above, we use it to skip over the function definition so that we do not accidentally execute the function before it is invoked, skipping to `[M+3]`, the `MKCLOSURE` command copies all of the shared pointers on the stack into a code environment, pushes a handle to the stack, and points to `[1]` in the program memory. The `ENTER` operation moves the frame base down the stack so that the closure can see its arguments, with the explicit understanding that on call, all of the necessary arguments will be provided either by the closure environment or the callee, on the stack. After that the function body is executed, then `NROT n+1` moves the final result under the arguments, then `DROP n` drops those. Finally `RET` pops a destination from the `return` stack and sets the program counter to it, then it sets the frame base to be what it was before the `ENTER` instruction was executed.

## A Simple Example

To see what a function definition and call site look like, consider a program which takes one argument and increments it by one:

```stack isa
[0]     JMP 72
[9]     ENTER 1
[18]    GET_LOCAL 0
[27]    PUSH 1
[36]    ADD
[45]    NROT 2
[54]    DROP 1
[63]    RET
[72]    MKCLOSURE 9
[81]    MKGLOBAL 15
[90]    LOADGLOBAL 15
[99]    PUSH 3
[108]   CALL 1
[117]   HALT
```
This corresponds to the following lisp code 
```lisp 
(define f 
    (lambda (x) 
      (+ x 1 )))
(f 3)
```
Let's look at the stack execution state. 
```text 
pc=0    op=JMP          stack=[]
pc=72   op=MKCLOSURE    stack=[]
pc=81   op=MKGLOBAL     stack=[0f]
pc=90   op=LOADGLOBAL   stack=[]
pc=99   op=PUSH         stack=[0f]
pc=108  op=CALL         stack=[0f, 3]
pc=9    op=ENTER        stack=[3]
pc=18   op=GETLOCAL     stack=[3]
pc=27   op=PUSH         stack=[3, 3]
pc=36   op=ADD          stack=[3, 3, 1]
pc=45   op=NROT         stack=[3, 4]
pc=54   op=DROP         stack=[4, 3]
pc=63   op=RET          stack=[4]
pc=117  op=HALT         stack=[4]
```

* First we `JMP` past the function body to `pc = 72`, where we make the closure, since this function only takes $1$ argument and doesn't close over anything, it's `CodeEnv.captured_vars` is empty. 
* Then we `MKGLOBAL 15`, that is, we take the value on the top of the stack, which right now is the closure handler, and add it to the heap with global name $15$. Now we can call this function from anywhere else in the code
* Then we add the function argument, in this case $3$, and `CALL 1`, the operand to the `CALL` instruction tells it how far down in the stack the function we're calling is. So we take that global variable, look inside and find a `CodeEnv` with `code_idx = 9`
* We follow that and pull `GET_LOCAL 0` into the stack, which is the first argument to the function, then push $1$, and `ADD`. Then we clean up the argument that was passed to the function, leaving only the $4$ on top, as desired.

## A Slightly More Complex Example

The prior example lacks two elements which are critical to a virtual machine which hopes
to host a lisp language: the closure list is empty, we dont demonstrate conditionals
and more importantly, the example is not recursive. To this end, let's see what the
classic `fib` implementation looks like on this machine. As a reminder, a recursive 
`fib` implementation looks something like this 
```lisp
  (define (fib n)
    (if (eq n 0) 0
      (if (eq n 1) 1
        (+ (fib (- n 1)) (fib (- n 2))))))
  (fib 2)
```
Which becomes this:
```ISA
[0] jmp 252
[9] enter 1
[18] get_local 0
[27] push 0
[36] eq
[45] cjmp 216
[54] get_local 0
[63] push 1
[72] eq
[81] cjmp 198
[90] loadglobal 15
[99] get_local 0
[108] push 1
[117] sub
[126] call 1
[135] loadglobal 15
[144] get_local 0
[153] push 2
[162] sub
[171] call 1
[180] add
[189] jmp 207
[198] push 1
[207] jmp 225
[216] push 0
[225] nrot 2
[234] drop 1
[243] ret
[252] mkclosure 9
[261] mkglobal 15
[270] loadglobal 15
[279] push 4
[288] call 1
[297] halt
```
