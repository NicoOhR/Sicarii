Every year, from December 1st to the 25th, a new programming puzzle is
published to adventofcode.com. Each puzzle contains in it two parts, and
are almost always some of the best designed puzzles I play during the
year. I typically solve Advant of Code in Haskell, since I find that the
strictly functional nature of the language encourages more interesting
solutions than the bruteforce calculations that are often possible. The
Haskell itch came a little early this year; coupled with the fact that it
didn't exactly sit right with me that I have never implemented a neural
network from scratch by myself, and the fact that Haskell is pretty
awesome for linear algebra stuff, as we will see, I figured it would be
fun to go through the implemntation of an NN in Haskell. This article
assumes no knowledge of haskell or machine learning, but does require some
knoweldge in calculus and linear algebra. I hope to achieve two things in
this article, firstly I want to demonstrate that the basic primitive of
the machine learning field, the neural network, is really not that
complicated or hard to get your head around, and secondly I would like to
answer the decently common question "why use haskell?".

## Setting Up

First, let's try to appreciate what it is exactly that we are building.
Put into a sentence, a neural network is just *"the composition of linear
transformations and non-linear functions"* and we typically tack on
*"which approximates a continuous function in $\mathbb{R}^n$"* implictily.
Let's try to tackle each part of this definition, I'll keep it brief, and
will try to add resources for anyone who wants to read more

* A *linear transformation* is any transformation which transforms objects
in a globaly consistent manner. For our purposes, that means that how much
input $x$ is transformed by linear transformation $W$ does not depend on
$x$ at all. In addition, we can add another vector after the
transformation while maintaining linearity, for a reason that we'll get to
in a minute, this is very useful.
* A *non-linear function* is all other transformation, that is, non-linear
function $f$ transforms $x$ differently depending on the specific value of
$x$
* A *continuous function in* $\mathbb{R}^n$ is a vector valued function
where a small variation of the input will lead to a similarily small
variation in the output. The exact details of continuity are not strictly
necessary to understand, and while I encourage everyone to suffer through
analysis as I have, we'll just move on for now.

Taken together, the neural network $g$ which takes as input $x$ which is
a vector in $\mathbb{R}^m$ and produces $y$ which is a vector in
$\mathbb{R}^n$, with non-linear function $f$, and linear transformations
$W^1, W^2,...W^L$ and vectors $b^1, b^2,... b^L$ is expressed as 

$$ g(x) = f(W^L(f(W^{L-1}f(...f(W^1x + b^1)))+b^{L-1})+b^L) $$

With the correct values of $W^i$ and $b^i$, together referred to as the
parameters of the neural network, and denoted as $theta$ (I pinky promise
this will probably be the only greek letter we use) we can approximate any
function $f$, kinda. Quick mathematical aside: there is a theorem, the
quite badly named universal approximation theorem, which states that for
any continuous function $f$, there exists an arbitrarily large linear
transformation $A$ and some $b$ such that for all $x$, $$||f(x) - g(x)||
< \varepsilon$$ We're in the real world, so we might not be able to
compute the "arbitrarily large" $A$, but this is rarely if ever a problem.

We'll introduce some more mathematical players later, when we need to
actually *find* our parameters. But for now lets do some programming.

## Haskell my sweet prince ( ˘ ³˘)♥

If you've never seen functional programming before some of this might be
an adjustment, but give me more time than its probably worth and I promise
you'll be more confused than when we started. Snark aside, one of my
favorite aspects of functional programming is that once you have a good
representation of your data, you've made some pretty decent progress
towards solving your problem.

So far, we have a non-linear function, which we call an *activation
function* and we have our parameters. 

```{Haskell}
data Theta = Theta [(Matrix Double, Vector Double)] deriving (Show)

type Activation = Double -> Double
``` 

`data Theta` is how we define a new type in Haskell, the `Theta` on the
other side of the definition is the data constructor of this type. While
not strictly necessary, I like using data constructors as a sort of built
in label for parameters which is helpful if you're, like me, bad about
naming your variables. ```[(Matrix Double, Vector Double)]``` means that
this is a list of the tuple of a matrix of doubles, and a vector of
doubles, our $W^i$ and $b^i$ respectively.

next is

