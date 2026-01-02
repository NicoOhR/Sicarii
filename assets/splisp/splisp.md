## Introduction 

Lisp is a language/concept/idea/philosophy that has been, by people a sizable margin older than me on the internet, sermonized to me on occasion. I've written a touch of it here and there, having started but never finished SICP, which I'll do one day soon I promise, and getting as far as 15 out of the famous 99 lisp problems. This is to say that I know enough lisp to understand why for a certain generation of programmers it was a synecdoche for software writ large. It's trivially easy to parse and writing an interpreter for it can famously be written in some 40 lines, in lisp itself no less! I've chosen to write a Lisp compiler for a couple of reasons, firstly, I've wanted to write a compiler for sometime; motivated by a basic hubris that I could do better, which I think all software people share to some extent. I was spurred into actually starting this project by Stevey's article, [Rich Programmer Food](https://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?) where this slightly bold claim: 

> If you don't know how compilers work, then you don't know how computers work

Being an unfortunately Kant-y "first principles" guy who painfully taught himself E&M because "it's embarrassing to be a computer person who doesn't understand how electricity works",I found Stevey's admittedly diatribal argument very persuasive. The language of choice for this escapade is C++ because I need some way to prove I can program in C++. I also felt that I haven't given C++ a fair chance, only having used it when a pre-existing project I have been vetted into used it or in class. Scout's honour, I will try to reserve overly opinionated judgements on the language to snarky remarks and side notes. This article will be organized in the order that I implemented these features in an effort to put together a narrative, as well as allow us to get right to programming and sprinkle theory along the way.

## Build Systems

Fudge. Before we can get to coding we have to contend with building the project, I don't say anything very interesting from a technical perspective here, none of this will be on the test. You almost forget that this isn't an entirely solved problem. In Python uv has solved any pip issue I've ever had, cargo is delightful, and even in Haskell cabal is serviceable, probably, I will not pretend to have attempted to deploy in Haskell. In C/C++ we still have to contend with the early architectural decision of "how should I build this?" 

For this project, we'll use CMake. Coming from embedded land, CMake is certainly the "lesser evil" compared to vendor provided IDEs. CMake is a makefile generator<label for="cmake" id="1" class="margin-toggle sidenote-number"></label><input type="checkbox" id="cmake" class="margin-toggle"/><span class="sidenote">This is kind of important, CMake is a meta build system, as in, it builds what builds your project. It is *not* what builds your project. Here, we'll just stick to makefiles, but I can vouch for Ninja</span>, and a domain specific language. It was created to fix "we have too many makefiles and these autoconf to manage multiple build types are getting out of hand!", and introduced "we have too many CMakeLists and these CMake scripts to manage multiple build types are getting out of hand!" 

Hey, I said I'll reserve judgement for C++, CMake is fair game. That quick rant aside, I'm using CMake here because I prefer interfacing with `gtest` through the `ctest` framework, and also I know how to use it and I didn't want to think terribly hard about build systems for this project. Maybe one day I'll revert to using a monolithic makefile, or give Bazel a go.

We'll split our program up into `src`, `lib` and `tests`. The majority of the code will be in `lib`, which will define each stage of the compilation process as it's own class, and use `src` to define the CLI which will ingest a file and output a program. The root level CMakeLists.txt, the file name that CMake uses to interact with your program like this

```CMake
cmake_minimum_required(VERSION 3.23)

project(
  splisp
  ...
)

include(CTest)

...
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_subdirectory(lib)
add_subdirectory(src)
if(BUILD_TESTING)
  add_subdirectory(tests)
endif()
```

The only gotcha here if you're doing this yourself is `CMAKE_EXPORT_COMPILE_COMMANDS` which you must turn on to get proper LSP support in your editor. The `add_subdirectory` directive tells CMake to "go look at that directory and execute the CMakeLists.txt in there". In the `lib/splisp_lib` directory, the CMakeLists look like this 

```CMake
add_library(splisp_lib STATIC
...
)

find_package(Boost 1.74 REQUIRED)
target_link_libraries(splisp_lib PRIVATE Boost::boost)

target_include_directories(splisp_lib 
  PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)
target_compile_features(splisp_lib PUBLIC cxx_std_20)

target_compile_options(splisp_lib PRIVATE
  $<$<CXX_COMPILER_ID:GNU,Clang,AppleClang>:-Wall -Wextra -Wpedantic>
  -g
  $<$<CXX_COMPILER_ID:MSVC>:/W4 /permissive->
)
```

I'll highlight two things here, one, we're using Boost, and I recommend anyone using C++ to add Boost, and secondly using generator expressions, what CMake calls the meta-commands like `$<INSTALL_INTERFACE:include>` will cost you some upfront complexity but simplify managing the CMake configuration down the line. I don't know if this is exactly the perfect project setup, but it really didn't seem like there was a decided upon community project template to work off of. Sorry, this article is about a lisp implementation, one day I'll write a programmer-polemic about CMake.

## Lexing

The Lexer, full name lexical analyzer, is generally the entry point of a compiler or interpreter. Sometimes there's a pre-processor which works on plain text, and if you've worked with C macros enough you probably winced just now. The lexer is responsible for taking the plain text characters and categorizes them into broad categories in the same that we process natural language by nouns, verbs, adjectives etc, our lexer will identify keywords and operations, collectively called tokens. For a lisp implemented for pedagogical reasons, there is not typically much of a reason to make a fully fledged lexer as a result of Lisp's simple syntax, which can be summarized as "code is data and data is code and all are lists", a motto we'll revisit when we get to the parsing and compiling stage. I like having the lexer since it simplifies the usually extensive parsing process and it's not too hard. I've opted to make the lexer into an iterator-like class, so for our public interface we'll only expose the `next()` and `peek()` methods:

```c++
class Lexer {
public:
  Lexer(std::string program);
  std::optional<Token> next();
  std::optional<Token> peek() const;

private:
  std::vector<Token> tokenized;
  ...
};
```

We'll be using `std::optional` rather often, justified as Rust-withdrawals, which more conveniently use the result of `next` and `peek` as conditionals, and generally just makes working with these functions a bit more explicit. For midsized projects like this, where several componenets have to get implemented before we can verify that the component Just Works (tm), unit tests give us some confidence in proceeding. Using `gtest`, we can define a few tests to make sure we're on the right track. This test verifies that we can lex basic tokens:

```C++
TEST(ParserTests, KeywordsAndBools) {
  Parser parser(Lexer("(if #t 1 0)"));
  ast::AST ast = parser.parse();

  ASSERT_EQ(ast.size(), 1U);
  ASSERT_NE(ast[0], nullptr);
  const auto *list = as_list(*ast[0]);
  ASSERT_NE(list, nullptr);
  ASSERT_EQ(list->list.size(), 4U);

  const auto *kw_item = list_item(*list, 0);
  ASSERT_NE(kw_item, nullptr);
  const auto *kw = as_symbol(*kw_item);
  ASSERT_NE(kw, nullptr);
  const auto *kw_val = std::get_if<ast::Keyword>(&kw->value);
  ASSERT_NE(kw_val, nullptr);
  EXPECT_EQ(*kw_val, ast::Keyword::if_expr);

  const auto *cond_item = list_item(*list, 1);
  ASSERT_NE(cond_item, nullptr);
  const auto *cond = as_symbol(*cond_item);
  ASSERT_NE(cond, nullptr);
  const auto *cond_val = std::get_if<bool>(&cond->value);
  ASSERT_NE(cond_val, nullptr);
  EXPECT_TRUE(*cond_val);

  const auto *then_item = list_item(*list, 2);
  ASSERT_NE(then_item, nullptr);
  const auto *then_val = as_symbol(*then_item);
  ASSERT_NE(then_val, nullptr);
  const auto *then_num = std::get_if<std::uint64_t>(&then_val->value);
  ASSERT_NE(then_num, nullptr);
  EXPECT_EQ(*then_num, 1U);

  const auto *else_item = list_item(*list, 3);
  ASSERT_NE(else_item, nullptr);
  const auto *else_val = as_symbol(*else_item);
  ASSERT_NE(else_val, nullptr);
  const auto *else_num = std::get_if<std::uint64_t>(&else_val->value);
  ASSERT_NE(else_num, nullptr);
  EXPECT_EQ(*else_num, 0U);
}
```

The above is really enough for a simple component like the Lexer which has very few invariants, with that as a goal to shoot for, we can start implementing the lexer. In most simple lisp implementations, the lexer adds white space to either side of `(` and `)`, and then splits the text by white spaces. This is the first step for us as well. 

```C++
Lexer::Lexer(std::string program) {
  // preprocess string
  boost::replace_all(program, "(", " ( ");
  boost::replace_all(program, ")", " ) ");
  boost::algorithm::trim(program);

  std::vector<std::string> words;
  boost::split(words, program, boost::is_space(), boost::token_compress_on);
  ...
}
```

Our tokens can be starting a list `(`, ending a list `)`, atomic, for this project this will be just numbers, or an identity. Identities (idents) are sometimes called symbols in the wider lisp world, I burrowed the name from Scheme, and basically describe non-lists which evaluate to something else, e.g. variable names. I went through the trouble of doing this since it allows us to use primitive operations as values which can be passed as values to higher order functions, like `fmap`. In an effort to avoid chaining `if` statements, I summarized these rules in an `unordered_map` and used `.find()` to get back the `TokenKind`. The map is defined in our header file

```C++
  const std::unordered_map<std::string, TokenKind> keywords = {
      {"(", TokenKind::lparn},  {")", TokenKind::rparn}, {"+", TokenKind::ident},
      {"-", TokenKind::ident},  {"*", TokenKind::ident}, {"/", TokenKind::ident},
      {"#t", TokenKind::atoms}, {"#f", TokenKind::atoms}};
```

And is used in the constructor like so

```C++
... Token curr;
for (std::string str : words) {
  curr.lexeme = str;
  auto match = keywords.find(curr.lexeme);
  // first we match tokens for exact matches
  if (match != keywords.end()) {
    curr.kind = match->second;
  }
  // everything else has to be a non kword ident or non bool atom, for now
  // we don't consider variable names
  else {
    curr.kind = TokenKind::atoms;
  }
  tokenized.push_back(curr);
}
...
``` 

The `lexeme` field of the `Token` struct is just the raw string of the token. With that, we have an iterator of `Tokens` which represent our program.

## Parsing

Now that we have the program in tokenized form, we start parsing the program. If lexing is recognizing individual words, we can think of parsing as organizing these words into sentences and paragraphs. Our goal is to take in tokens and output an Abstract Syntax Tree (AST). In general, tree's do a good job of capturing the meaning of *language-like things* because they describe the flow of composition naturally, where leaves describe basic objects and the intermediate nodes are "compose children by this rule". Extending this philosopical meandering going for a little bit, strings are linear and create meaning through composition, composition is recursive, and recursive structure is a tree.<label for="language" id="2" class="margin-toggle sidenote-number"></label><input type="checkbox" id="language" class="margin-toggle"/><span class="sidenote">I will not be contending with a Deluzian approach to compiler design today, though I guess DAGs are rhizomatic in a manner.</span> The AST is especially apt at describing a lisp because the basic object of a lisp, the S-expression, is in itself treelike. As I mentioned earlier, in lisp "code is data and data is code and all are lists", while that's philosophically accurate, it would be more accurate to say that "all are S-expressions", abbreviated as `sexp`. A S-expression is either an atom, $x$, or an expression $(x . y)$ where both $x$ and $y$ are S-expressions themselves, in modern syntax we usually just drop the $.$ all together. So all together, we have `Symbol`s which basic atoms of the language numbers, bools, keywords, we have `List`s which are a collection of `SExp`s, and we have `SExp`s which are either a `List` or a `Symbol`. The recursive definition of `SExp` is one of the more satisfying aspects of the language, <label for="rust" id="3" class="margin-toggle sidenote-number"></label><input type="checkbox" id="rust" class="margin-toggle"/><span class="sidenote">And also the only time I was actively glad to not be using Rust during this project</span>and likely the source of the taoist imagery that's often associated with lisp.

Let me take a quick aside to explain the syntax of lisp, it doesn't take very long. Lisp uses *prefix notation* also known as *polish notation*, where instead of a binary operation being *between* two operands, it is *before* operands. So in lisp the computation `(1 + 1)` is notated as `(+ 1 1)`. This simplifies the parsing process, since we don't have to consider the precedence of an operator, computing `(2 * 1 + 1)` would require us to recognize that `*` happens before `+`, using prefix notation `(+ 1 (* 2 1))` localizes the syntax and removes ambiguity. The other rule, and the only one we'll have to think about for a while is that `(` starts a list and `)` ends a list. That's about the size of it. 

So how do we actually parse this? There's a number of ways to parse a string into a tree, but in the process of immanent critique it's been decided that *recursive decent* strikes the happy middle between being easy to implement and not having an obscene run time. First we have to start with a grammar rules, recall again that we're only concerning ourselves with lists, so in effect our rules look like:  

```
sexp := Symbol | list; 

list := "(" { sexp }  ")"
```

Since the definitions are mutually recursive, we can append to our tree and then recur until we hit a symbol.

The grammar rules are translated into C++ with the help of forward declaration

```C++

struct Symbol {
  std::variant<Keyword, std::string, std::uint64_t, bool> value;
};

struct SExp;

struct List {
  std::vector<std::unique_ptr<SExp>> list;
};

struct SExp {
  std::variant<List, Symbol> node;
};
```

Now that we have the representation set up, we should probably also create the tests which will verify it:

```C++
TEST(ParserTests, KeywordsAndBools) {
  Parser parser(Lexer("(if #t 1 0)"));
  ast::AST ast = parser.parse();

  ASSERT_EQ(ast.size(), 1U);
  ASSERT_NE(ast[0], nullptr);
  const auto *list = as_list(*ast[0]);
  ASSERT_NE(list, nullptr);
  ASSERT_EQ(list->list.size(), 4U);

  const auto *kw_item = list_item(*list, 0);
  ASSERT_NE(kw_item, nullptr);
  const auto *kw = as_symbol(*kw_item);
  ASSERT_NE(kw, nullptr);
  const auto *kw_val = std::get_if<ast::Keyword>(&kw->value);
  ASSERT_NE(kw_val, nullptr);
  EXPECT_EQ(*kw_val, ast::Keyword::if_expr);

  const auto *cond_item = list_item(*list, 1);
  ASSERT_NE(cond_item, nullptr);
  const auto *cond = as_symbol(*cond_item);
  ASSERT_NE(cond, nullptr);
  const auto *cond_val = std::get_if<bool>(&cond->value);
  ASSERT_NE(cond_val, nullptr);
  EXPECT_TRUE(*cond_val);

  const auto *then_item = list_item(*list, 2);
  ASSERT_NE(then_item, nullptr);
  const auto *then_val = as_symbol(*then_item);
  ASSERT_NE(then_val, nullptr);
  const auto *then_num = std::get_if<std::uint64_t>(&then_val->value);
  ASSERT_NE(then_num, nullptr);
  EXPECT_EQ(*then_num, 1U);

  const auto *else_item = list_item(*list, 3);
  ASSERT_NE(else_item, nullptr);
  const auto *else_val = as_symbol(*else_item);
  ASSERT_NE(else_val, nullptr);
  const auto *else_num = std::get_if<std::uint64_t>(&else_val->value);
  ASSERT_NE(else_num, nullptr);
  EXPECT_EQ(*else_num, 0U);
}
```

Here, the functions `list_item`, `as_symbol`, and `as_list` are helper functions which strip the `std::variant` and return back the struct itself. This functionality is not really necessary outside of testing the parser, so we relegate it to the test's namespace. We want our compilation process to stop here if the program contains any issues, so we include two obvious ones here, mismatched parentheses, and invalid atoms:

```C++
TEST(ParserTests, MismatchedParensThrows) {
  Parser parser(Lexer(")"));
  EXPECT_THROW((void)parser.parse(), std::logic_error);
}

TEST(ParserTests, InvalidAtomThrows) {
  Parser parser(Lexer("foo"));
  EXPECT_THROW((void)parser.parse(), std::invalid_argument);
}
```

A quick note on testing methodology, you might have noticed I am only testing the public methods for both the Lexer and the Parser, this is because I am almost certain that the implementation details for the internals might change as we introduce more complicated "nice to haves" later on, like error reporting or debugging capabilities.

Now that we can forge ahead with some confidence, we can finally implement recursive decent parsing with a relatively simple procedure: if the next value is `(`, then we go into the `create_list()` routine. In the `create_list()` routine, until a `)` is encountered, we create new S-expressions recursively by calling `create_sexp()`. These two methods will call each other until an atom is encountered, which is pushed into the list and continue the list creation loop. I genuinely love recursive solutions to things, stack safety and performance be damned.

```C++
std::unique_ptr<SExp> Parser::create_sexp() {
  Token next = lex.next().value();
  switch (next.kind) {
  case (TokenKind::lparn): {
    SExp sexp = {.node = create_list()};
    return std::make_unique<SExp>(std::move(sexp));
  }
  case (TokenKind::atoms): {
    SExp sexp;
    auto num = is_number(next.lexeme);
    if (num) {
      sexp = {.node = Symbol{num.value()}};
      return std::make_unique<SExp>(std::move(sexp));
    };
    // identical procedure to the above is repeated for keywords and bools...
    throw std::invalid_argument("invalid atom");
    break;
  }
  case (TokenKind::rparn): {
    throw std::logic_error("mismatched parantheses");
    break;
  }
  case (TokenKind::ident): {
    SExp sexp = {.node = Symbol{next.lexeme}};
    return std::make_unique<SExp>(std::move(sexp));
  }
  }
  throw std::logic_error("strange construction");
}

List Parser::create_list() {
  List ret = List();
  while (lex.peek().value().kind != TokenKind::rparn) {
    ret.list.push_back((create_sexp()));
  }
  lex.next();
  return ret;
}
```

The above passes the tests with flying colors, but we'd still like to verify visually what our AST looks like. Printing the AST is somewhat complicated by the fact that we used `std::variant`, reason being is that there is not a built in formatter for the container, so we have to either chain `std::get_if<T>` statements or use the `std::visit` method which applies the visitor pattern to the variant, we choose the former rather than the later for reasons we'll explain in a second.

```C++
void print_symbol(const Symbol &sym, int level) {
  std::visit(
      [level](const auto &v) {
        using T = std::decay_t<decltype(v)>;
        std::string stuff(level * 2, ' ');
        if constexpr (std::is_same_v<T, std::string>) {
          std::cout << stuff << "Ident " << v;
        } else if constexpr (std::is_same_v<T, std::uint64_t>) {
          std::cout << stuff << "Int " << v;
        } else if constexpr (std::is_same_v<T, bool>) {
          std::cout << stuff << "Bool " << (v ? "true" : "false");
        } else if constexpr (std::is_same_v<T, Keyword>) {
          std::cout << stuff << "Kword " << print_keyword(v);
        }
      },
      sym.value);
}
```

Despite using `std::get_if<T>` inside the visitor anyway, the benefit of `std::visit` is that it *forces* us to implement a case for every possible type. So, if we introduce another thing that symbol can be, the program would fail to compile until we handle it in the print as well. The purpose of using `decay_t<decltype(t)>` is a little complex but boils down to the fact that since we're passing `v` by a constant reference to avoid copying, `std::decay_t` allows us to match the type concretly, `std::decay_t` strips the reference and gives us back what we're actually interested in matching against<label for="decay" id="3" class="margin-toggle sidenote-number"></label><input type="checkbox" id="decay" class="margin-toggle"/><span class="sidenote">More percisely, `decay_t` strips the *const and volatile qualifications* and references. The docs refer to this as "perform type conversions equivalent to the ones performed when passing to a function by value".</span>. That is, instead of checking if the `std::is_same_v<T, std::uint64&>`, we simply check `std::is_same_v<T, std::uint64>`. I feel like surely `std::variant` is worthy of having a built in formatter to hide this implementation from a frankly disinterested user, but there's probably some edge cases that need to be left up to the user which prevent this from being easily printable.

## Simple results

Trying all of this out in a simple main function, 

```C++
#include <frontend/lexer.hpp>
#include <frontend/parser.hpp>
#include <iostream>
#include <string>

int main() {
  std::string program = "(if #t (+ 3 4) 5)";
  Lexer lex(program);
  Parser parser(std::move(lex));
  auto ast = parser.parse();
  std::cout << "--+--" << std::endl;
  ast::print_ast(ast);
  return 0;
}
```
Will output:
```
AST
  List
    Kword if
    Bool true
    List
      Ident +
      Int 3
      Int 4
    Int 5
```

Exactly as expected. From here we need to build the code generator which will take the AST and convert it into what will actually run on the machine. Ah. That sounds hard. Generating x86 assembly is not impossible especially for simple operations like we're planning to do, but part of what I wanted out of this project is an animated output that will visualize the program through the compilation steps (lexer, parser, assembly, stack) which is a little hard to do if we're running on actual hardware. So next article, I'll go through the stack based virtual machine that I implemented, and finally for part three we'll discuss generating assembly from the AST.
