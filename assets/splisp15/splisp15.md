In the last article, we went through the construction of the parser and lexer for the Lisp compiler. While those componenets, as they were, would have worked for a lisp-y claculator, they lacked a very important feature(s) that would be required to make a more fully featured language. Lisp has a concept of a "special form", or a list with additional meaning to it, these forms are still lists, so the mental model that "everything is a list" is still correct, but these allow us to interpret control flow and define functions/lambdas/variables. These special forms are primitive to the evaluator (in an interpted lisp) or the compiler (in our case). Macros allow the programmer to essentially introduce new special forms into the language, and those are evaluated before runtime, while much of the expressive power of Lisp comes from these macros, we will not be implementing those in the near future.

What changes are needed to introduce these into our AST? First, we need to define the node structure that represents the particular special form<label for="special" id="1" class="margin-toggle sidenote-number"></label><input type="checkbox" id="special" class="margin-toggle"/><span class="sidenote">`if` is relatively easy to lower without the use of a special form, so I elected to not implement it. For error checking reasons I'll go back and implement the `if` special form later, but for this article we'll only worry about implementing the `function` special form, which covers functions, lambdas, and let bindings</span> For the `function` special form that struct looks like this:

```c++
struct Function {
  std::optional<std::string> name;
  size_t scope_id;
  std::unique_ptr<SExp> args;
  std::unique_ptr<SExp> body;
};
```

The name of the function is optional since we'd like to be able to define lambdas as well. Largely speaking, there is not much of a difference between lambdas and functions at this level, we'll get more into the intercies of function calling when we do the name-resolution and closure conversion steps in the next article (hopefully). Remember that, in our parser, we are already matching the special `idents` against a `Keywords` enum, so our keywords already appear as identifiable nodes in the AST. It's possible to resolve those keywords as we encounter them, but since this is functionally a search and replacet ask, with $\mathcal{O}(1)$ complexity, I felt that the additional run time was a reasonable sacrifice for a simpler implementation. This way, the transformation we're implementing is `Valid AST -> Valid AST` and not `Possibly Invalid Tokens -> Valid AST`. So after we make our AST, we'll iterate over the top level `sexps` and call `resolve_forms` on them:

```c++
AST Parser::parse() {
  AST ast;
  while (this->lex.peek()) {
    ast.push_back(create_sexp());
  }
  for (auto &sexp : ast) {
    resolve_forms(*sexp);
  }
  return ast;
}
```

The `resolve_forms(SExp &sexp)` takes a reference to a `SExp` and, using the visitor pattern that we discussed in part 1, we look through the 


