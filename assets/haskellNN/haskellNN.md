{{{assets/haskellNN/training.html}}}

# Overview

## Recruiter TL;DR

I implemented a multilayer perceptron, using ReLU as the activation function, in
Haskell from scratch (with the exception of linear algebra operations). In the
first half, I detail the mathematical grounding of neural networks, and in the
second half walk through the implementation. Above you see the plot of the
learned function in blue and the actual function in red. The resulting neural
network is able to learn trigonometric functions, logical functions (AND, XOR,
OR etc.), with more yet to be tested.

## Wait, why?

It never exactly sat right with me that I have never implemented a neural
network from scratch by myself, being a Math/CS guy (tm). Additionally as we
will see Haskell is pretty awesome for linear algebra stuff, so I figured it
would be fun to go through the implementation of a neural network in Haskell.
This article assumes no knowledge of Haskell or machine learning, but does
require some knowledge in calculus and linear algebra, and that you are familiar
with *a* programming language. I hope to achieve two things in this article,
firstly I want to demonstrate that the basic primitive of the machine learning
field, the neural network, is really not that complicated or hard to get your
head around, and secondly I would like to answer the decently common question
"why use Haskell?".

# The Mathematics

## Setting Up

First, let's try to appreciate what it is exactly that we are building. Put into
a sentence, a neural network is just *"the composition of linear transformations
and non-linear functions"* and we typically tack on *"which approximates a
continuous function in $\mathbb{R}^d$ "* implicitly. Let's try to tackle each
part of this definition, I'll keep it brief, and will try to add resources for
anyone who wants to read more

- A *linear transformation* is any transformation which transforms objects in a
  globally consistent manner. For our purposes, that means that how much input
  $x$ is transformed by linear transformation $W$ does not depend on $x$ at all.
  In addition, we can add another vector after the transformation while
  maintaining linearity, for a reason that we'll get to in a minute, this is
  very useful.
- A *non-linear function* is all other transformation, that is, non-linear
  function $\sigma$ transforms $x$ by different amounts depending on the
  specific value of $x$
- A *continuous function in* $\mathbb{R}^d$ is a vector valued function where a
  small variation of the input will lead to a similarly small variation in the
  output. The exact details of continuity are not strictly necessary to
  understand, and while I encourage everyone to suffer through analysis as I
  have, we'll just move on for now.

Taken together, the neural network $g$ which takes as input $x$ which is a
vector in $\mathbb{R}^m$ and produces $y$ which is a vector in $\mathbb{R}^d$,
with non-linear function $\sigma$, and linear transformations
$W^{(1)}, W^{(2)},...W^{(N)}$ and vectors $b^{(1)}, b^{(2)},... b^{(N)}$, over
$N$ layers, is expressed as

$$ g(x) = \sigma(W^{(N)}(\sigma(W^{(N-1)}\sigma(...\sigma(W^{(1)}x + b^{(1)})))+b^{(N-1)})+b^{(N)}) $$

With the correct values of $W^{(n)}$ and $b^{(n)}$, together referred to as the
parameters of the neural network, and denoted as $\theta$ (I pinky promise this
will probably be the only Greek letter we use) we can approximate any function
$h$.

## Finding the Parameters

In optimization, we generally like thinking about minimization problems rather
than maximization problems, more by convention than by any other reason. In this
case we're trying to find the parameters which are *least wrong* which is the
same as the parameters that are *most right*. This is expressed notationally as:

$$
\operatorname*{argmin}_\theta ||f(x) - g_{\theta}(x)||
$$

And is read as "find $\theta$ such that the difference between the function $f$
and the function parameterized by $\theta$, $g$, is minimized". In this case, we
can use the standard euclidean norm to compute the difference between our
functions:

$$
||v|| = \sqrt{v_1^2 + v_2^2 + ... v_d^2}
$$

Sometimes, depending on your problem and what the output of your network is,
other *metrics* should be used (for example, for probability distributions, we
use the KL-divergence metric to quantify how far our network is from our
function). Because we'll do the chosen metric operation a lot, we'd like it to
be computationally cheap, and while the square root operation has become close
to constant time as hardware and compiler optimizations have advanced, we would
still like to not do it on every iteration. We can avoid doing the square root
by recognizing that, because the square function is strictly increasing for
positive values!!!which a distance always will be!!! minimizing the square of
the norm is exactly the same as minimizing the norm itself, so, our goals
becomes:

$$
\operatorname*{argmin}_\theta~ ||f(x) - g(x)||^2
$$

Or more explicitly

$$
\operatorname*{argmin}_\theta~ (f(x)_1 - g(x)_1)^2+(f(x)_2-g(x)_2)^2+...(f(x)_d - g(x)_d)^2
$$

We call the measure of how "wrong" our network is the loss function (also cost,
depending on whose writing the paper) and we denote it by $L$. Loss is
parameterized by both $\theta$ and $x$. We would like to find the $\theta$
which, on average, minimizes $L_\theta$. How do we find these $\theta$? Here we
introduce the gradient, which is, for a vector valued function $f: \mathbb{R}^d
\to \mathbb{R}$ defined as

$$
\nabla f(\mathbf{x}) =
\begin{pmatrix}
\frac{\partial f}{\partial x_1}(\mathbf{x}) \\
\frac{\partial f}{\partial x_2}(\mathbf{x}) \\
\vdots \\
\frac{\partial f}{\partial x_d}(\mathbf{x})
\end{pmatrix}
$$

I'll ask you to take the following on axiom, the gradient of $f$, $\nabla f$
evaluated at $x$ is the direction of fastest change positive change, and
$-\nabla f$ is the direction of fastest negative change. To demonstrate this at
least visually, consider the following function, from $\mathbb{R} \to
\mathbb{R}$

{{{assets/haskellNN/gradient_2d.html}}}

The relevant part here is to notice that the direction of the gradient, positive
or negative, will always point towards the nearest highest point, that is, the
local maxima.

For a slightly less trivial example, we can inspect this phenomena in a function
from $\mathbb{R}^2 \to \mathbb{R}$.

{{{assets/haskellNN/gradient_3d.html}}}

This pattern is unsurprisingly consistent in higher dimensions, the gradient
will always point towards the top of the hill.

So if we follow the negative gradient of the loss function at every point, we'll
eventually find *a* minimum! Importantly, this is not the global minimum, only a
local minimum, where moving a little bit in any direction would increase the
function.

So to find the locally optimal $\theta$, we find the negative gradient of the
loss function, with respect to $\theta$ for each $x$ in our data set, and move
along the direction of the gradient.

## Backprop

The way we actually get this gradient is through an algorithm that in machine
learning is called *backpropagation* often just *backprop* for short.!!!Through
seemingly convergent evolution, other fields which require derivatives of
computations have invented similar algorithms: control/engineering people refer
to it as backwards mode differentiation, finance people will call it adjoint
algorithmic differentiation!!! Backpropagation is a type of automatic
differentiation which on a practical level is just a clever application of the
chain rule for multivariate derivatives. A rigorous treatment of the algorithm
which I suspect most people interested in machine learning have already read can
be found [here](http://neuralnetworksanddeeplearning.com/index.html) , written
by Michael Nielsen. I will be taking a slightly different route, though
ultimately we arrive at the same conclusion.

Let's bring back the definition of the neural network we had, then apply the
loss function to it.

$$
L = \mathrm{Loss}(\sigma(W^{(N)}(\sigma(W^{(N-1)}\sigma(...\sigma(W^{(1)}x +
b^{(1)})))+b^{(N-1)})+b^{(N)}))
$$

We write $z^{(n)}$ for the output of layer $n$ *before* it goes through the
activation function, and $a^{(n)} = \sigma\!\left(z^{(n)}\right)$ for the result
of passing it through, so that $a^{(N)}$ is the network's actual output and
$a^{(0)} = x$ is its input. We write the linear part of layer $n$ as

$$ A^{(n)}(a) := W^{(n)} a + b^{(n)} $$

Using this notation, the loss is a function of the pre-activation $z^{(n)}$ of
some intermediate layer $n$. Feed $z^{(n)}$ through the activation, then through
the rest of the network, then into the loss:

$$
L = \mathrm{Loss}(\sigma(A^{(N)}(\sigma(A^{(N-1)}(\cdots
\sigma(A^{(n+1)}(z^{(n)}))\cdots)))))
$$

Whenever we have to deal with deeply nested functions such as this, we tend to
abbreviate it with $\circ$, which is defined as $f \circ g = f(g(x))$, this lets
us avoid hard to manage parenthesization. So we can more easily express our loss
function as:

$$
L = \mathrm{Loss} \circ \sigma \circ A^{(N)} \circ \sigma \circ A^{(N-1)} \circ
\cdots \circ \sigma \circ A^{(n+1)}\left(z^{(n)}\right)
$$

The multivariate chain rule states the derivative of a composition $$h = h_1
\circ \cdots \circ h_k$$ is the composition of the derivatives of its pieces
!!!To fully appreciate this would require a little bit of time invested in
multivariate calculus and probably a little bit of analysis (and a bit of
category theory if you ask me), so we'll be skipping the proof for now. The main
take away is that we have a closed form way to take the derivative of a function
made up of other functions.!!!:

$$ Dh = Dh_1 \cdot Dh_2 \cdots Dh_k $$

We already know each individual component. The derivative of $A^{(n)}$ is just
$W^{(n)}$, and because $\sigma$ is applied elementwise, its derivative is a
diagonal matrix:

$$
D\sigma\!\left(z^{(n)}\right)
= \mathrm{diag}\!\left(\sigma'\!\left(z^{(n)}\right)\right)
$$

Stringing those together from the output backwards gives

$$
\nabla_{z^{(n)}} L^{T} = \nabla_{a^{(N)}} L^{T} \,
D\sigma\!\left(z^{(N)}\right) \, W^{(N)} \, D\sigma\!\left(z^{(N-1)}\right) \,
W^{(N-1)} \cdots D\sigma\!\left(z^{(n+1)}\right) \, W^{(n+1)}
$$

Notice that the product for layer $n$ is the product for layer $n+1$ with two
more factors tacked on the end. So we can take the partial products up to $n$ to
get the derivatives we're interested in. In most write ups, this partial product
is named $\delta$, and is defined recursively:

$$ \begin{aligned} \left(\delta^{(N)}\right)^{T} &:= \nabla_{a^{(N)}} L^{T} \,
D\sigma\!\left(z^{(N)}\right) \\ \left(\delta^{(n)}\right)^{T} &:=
\left(\delta^{(n+1)}\right)^{T} \, W^{(n+1)} \, D\sigma\!\left(z^{(n)}\right)
\end{aligned} $$

Transposing the recurrence, and using the fact that multiplying by a diagonal
matrix is the same as multiplying elementwise, we get the form that is typically
implemented

$$ \begin{aligned} \delta^{(n)} &= D\sigma\!\left(z^{(n)}\right)
\left(W^{(n+1)}\right)^{T} \delta^{(n+1)}\\ &= \sigma'\!\left(z^{(n)}\right)
\odot \left(W^{(n+1)}\right)^{T} \delta^{(n+1)} \end{aligned}$$

Once we have the $\delta$s, the gradients we actually wanted fall out
immediately:

$$ \begin{aligned} \frac{\partial L}{\partial W^{(n)}} &= \delta^{(n)}
\left(a^{(n-1)}\right)^{T}\\ \frac{\partial L}{\partial b^{(n)}} &= \delta^{(n)}
\end{aligned} $$

That was really quite complex, so don't feel bad if you feel like you god lost
somewhere along the way. I personally most definitely did not understand it the
first time. I suggest seeking other write ups as seeing the same mathematical
processes from multiple angles is often what makes them "click". The main take
aways from this section are:

- Neural Networks compose linear and non linear functions which are
  parameterized by a vector of values
- By setting a criterion called the Loss, we can test our network for
  correctness
- By following the gradient of this loss function in parameter space, we can
  minimize it, getting more correct values every iteration
- We find the gradient by a process called backpropogation

With that, we're ready to move on to the programming part of this write up.

# The Implementation

## Haskell my sweet prince ( ˘ ³˘)♥

If you've never seen functional programming before some of this might be an
adjustment, but give me more time than its probably worth and I promise you'll
be more confused than when we started. Snark aside, one of my favorite aspects
of functional programming is that once you have a good representation of your
data, you've made some pretty decent progress towards solving your problem.

So far, we have a non-linear function, which we call an *activation function*
and we have our parameters.

```Haskell
data Theta = Theta [(Matrix Double, Vector Double)] deriving (Show)

type Activation = Double -> Double
```

`data` is how we define a new type in Haskell, the `Theta` on the other side of
the definition is the data constructor of this type. While not strictly
necessary, I like using data constructors as a sort of built in label for
parameters which is helpful if you're, like me, bad about naming your variables.
`[(Matrix Double, Vector Double)]` means that this is a list of the tuple of a
matrix of doubles, and a vector of doubles, our $W^{(n)}$ and $b^{(n)}$
respectively. `deriving (Show)` tells the compiler that we want this type to be
show-able, which in this case just means that it can be coerced into a string so
we can print it. When we can or cannot derive a *typeclass* (what we call
characteristics) is dependent on a couple of things, but `Show` is relatively
simple I thing. If you've programmed in Rust before, this is largely where
`#[derive(Debug)]` comes from. The `type` keyword defines a synonyms for a type,
as opposed to `data` which defines a *new* type. In this case, it's just nicer
to give this type a name.

Eventually, we'll need to subtract thetas and scale them by a `double` type. Of
course, this is exactly defining a vector space over the theta type, and we
could probably define a vector space typeclass to get some nice binary
operations to do this, but this is just as achievable, and probably easier, to
just do with functions.

```Haskell
scaleThetas :: Theta -> Double -> Theta
scaleThetas (Theta ts) eta = Theta (fmap (\(x, y) -> (scale eta x, scale eta y)) ts)

subtractThetas :: Theta -> Theta -> Theta
subtractThetas (Theta ts) (Theta gs) = Theta (zipWith (\(x, y) (u, v) -> (x - u, y - v)) ts gs)
```

The double colon, `::` should be read as "has type of".!!!Exactly why there
isn't a distinction between what are procedurally thought of as the parameters
and the output of the function is an iota outside of the scope of this article
unfortunately, but the keyword to google here is *currying* (maybe like the
food, I never asked).!!! First, looking at the `scaleThetas` function, we take
as input a theta and a double, then fmaps the anonymous function
`(\x,y) -> (scale eta x, scale eta y)` on the `Theta`. `fmap` stands for
"function map", and does just that, given a function and a list, return the list
with the function applied to each element of the list, and `scale` is provided
by the Vector and Matrix types to do scalar multiplication. Looking at the
`subtractThetas` function, `zipWith` is a combination of `fmap` and `zip`, so
we're combining two lists, the inputs `ts` and `gs`, then applying the lambda
function `\(x,y) (u,v) -> (x - u, (y - v))` to that combined list. Let's write
the function for running running our neural network, that is, of obtaining an
approximation $\hat y$ for an input $x$. Remember that our network is just a
composition of matrices, vector additions, and activation functions which we
apply iteratively, this is rather cleanly expressed by the `fold` function which
Haskell provides us with.

If mathematical notation is more your jive, then the basic procedure that `fold`
describes can be interpreted as:

$$\text{fold}~f~a~\mathbf{b} = f(...f(f(f(a, b_1),b_2),b_3) ... b_k)$$

where $\mathbf{b}$ is a list of $k$ numbers. Maybe you can already see how this
applies to our neural network, seeing as how this looks very much like our
definition of a neural network. !!!The google term for further reading on the
categorical approach to this type of function is *catamorphism* for those of us
inclined to abstract nonsense.!!!

Interpreted as concretely as possible, the `fold` function takes a binary
operator, an accumulator and a list, applies the operator to each element in the
list with the accumulator, in order. It might help to think of it in more
programming-y terms:`foldl` "reduces" the list starting from the left, and has a
type of `foldl:: Foldable t => (b -> a -> b) -> b -> t a -> b`. `Foldable t` is
a restriction on the type of `t`, meaning that whatever `t a` is, it should be
foldable over, this is another example of a typeclass, like `Show` from earlier.
In our case, we'll be iterating over our $\theta$ which has the list type, which
is foldable over. Next, we see from the type that we want a function
`(b -> a -> b)` that is, it takes a type `b` and a type `a` and gives back a
type `b`, this is our binary operator. Then we also require a `b`, which is our
accumulator value, and finally the list `t a` which we process, returning a `b`.
. Recall our mathematical definition of a neural network:

$$ g(x) = \sigma(W^{(N)}(\sigma(W^{(N-1)}\sigma(...\sigma(W^{(1)}x + b^{(1)})))+b^{(N-1)})+b^{(N)}) $$

We want to end up with an output vector, so `b` is already determined to be of
type `Vector Double`, and we want to process over our `Theta`, so `a` naturally
becomes the tuple `(Matrix Double, Vector Double)`. If we keep our activation
function $\sigma$ consistent across all layers, the above formula is expressed
rather cleanly as

```Haskell
forward :: Network -> Vector Double -> Vector Double
forward (Network (Theta ts) _ f _) x0 =
    foldl step x0 ts
  where
    step :: Vector Double -> (Matrix Double, Vector Double) -> Vector Double
    step x (w, b) = cmap f (w #> x + b)
```

`#>` is just the Matrix-by-vector multiplication that `hmatrix`, the linear
algebra library I tend to default to, uses. As nice as this is, if you're
familiar with neural networks you might be wrinkling your nose a little right
now. Firstly, we'll often not apply an activation layer to our last layer to
ensure that our network can produce the entire range of values we're interested
in, so we'll need to special case our last layer. Additionally, recall from the
backpropagation equations, we need *all* $z^{(n)}$, not just the final output.
But never fret reader, any problem is really just a teachable moment, and we'll
use this particular problem to learn about monads!

## Monoids in the Category of Endofunctors

One of the more famously unhelpful sentences in programming is the answer to the
question "What is a monad?" which canonically is "Well simple! It's just a
monoid in the category of endofunctors! Of course!". This definition !!!This is
the secondary definition from a category theory perspective, and should probably
not be admitted into the conversation at all from the functional programming
perspective.The story goes that this result/definition was first published in
*Categories for the Working Mathematician* by Mac Lane, and then popularized as
a sardonic remark attributed to Phil Wadler (who introduced monads into early
Haskell) in James Iry's seminal blog *A Brief, Incomplete, and Mostly Wrong
History of Programming Languages*. Supposedly, when Wadler read the article he
responded in the mailing list "I did not know this, does someone have a
proof?"!!! is mostly a joke. There are countless blogs and videos (I recommend
the sheafification of g's video on the subject personally) trying to explain
monads; it's a rite of passage for functional bros I think. In a sentence, a
monad is a functor $T$ (a function which operates on types) which has an
associated transformations $\eta, \mu$ (things that operate on the functor)
which are associative and unital respectively, we won't spend more time on this
definition today. Today, we'll focus on only the monad we care about, the
`Writer` monad, to demonstrate how you would actually use one of these things in
the field. I will quote Kwang's excellent article on the writer monad to define
it

> "The Writer monad represents computations which produce a stream of data in
> addition to the computed values"

This definition is a little funny because, in many ways, this is what *all*
monads do; encode additional "side effects" of otherwise pure code. When in the
context of the writer monad, we can use `tell` (this is our associative natural
transformation $\eta$) to add to our list of logs, and `pure` to give back our
actual value (this is our unital natural transformation $\mu$). The writer monad
allows us to turn a function that would otherwise look like this

```{Haskell}
functionWithLogsPure :: inputType -> (outputType, [logType])
```

To a function that looks like this:

```{Haskell}
functionWithLogsWriter :: inputType -> Writer logType outputType
```

While also affording us the common operations we expect from logging
functionality. This point was something that took me a long time to get into my
brain, inside the context of a purely functional language like Haskell, monads
do not necessarily afford you any additional computational power, rather they
more naturally allow us to express computations algebraically. Consider
`functionWithLogsPure`, and how you would actually use it. You would supply it
with an argument, and get back an output and a log tuple. Then to use those with
another function which produces logs, you'd break up the tuple, creating an
intermediary log variable and intermediary output variable, then using that as
the input of the second function, you'd get back an output and another log which
you append together with the one you got back from the first call. While this is
not a complex operation, it is unfriendly, and gives room for error, a
programmer calling our function could unwittingly drop the logs, for example.
The Writer monad essentially encodes this operation in a generic way. In other
languages we would probably just include some state to handle this, allowing
functions to read and write from a piece of state, but this introduces other
complexities. This example demonstrated how we can use monads to encode state,
but it's a more general tool than that; monads simply allow us to package a
value with context, and gives us tools to wrap a value in that context and
compose contexts together.!!!On a more mathematical level, monads give shape for
the action of combining things, this structure lets us describe abstractly what
an algebra looks like.!!!

Back to our implementation, a slight hitch in this is that `foldl` is a pure
function, which means that the type checker will get very mad at you if you try
to use monads with it, luckily we have `foldlM` which allows us to pass monadic
functions to it. Implementing the writer monad into our naive `forward` we get

```Haskell
forwardW ::
    Network ->
    Vector Double ->
    Writer (DL.DList (Vector Double)) (Vector Double)
forwardW (Network (Theta ts) _ f _) x1 = do
    let (hiddenLayer, lastLayer) = (init ts, last ts)
    h <- foldM step x1 hiddenLayer
    let (wL, bL) = lastLayer
        y_hat = wL #> h + bL
    tell (DL.singleton y_hat)
    pure y_hat
  where
    step :: Vector Double -> (Matrix Double, Vector Double) -> Writer (DL.DList (Vector Double)) (Vector Double)
    step x (w, b) = do
        let z = cmap f (w #> x + b)
        tell (DL.singleton z)
        pure z
```

(Authors note: This is wrong now that I look at it, we should be `tell`ing the
preactivated `z`s, with ReLU the bug is functionally invisible but does need to
be fixed)

Every step of the `foldlM`, we `tell` to our List the resulting `z`, and pass to
the next call `pure z`. The resulting output of the `fowardW` function is
`[y_hat, [a_1, a_2, a_3 ... a_N]]` (`DList` is an interesting data structure
that I will not be reviewing today, think of it simply as a list). The last
piece of the puzzle before we can train our neural network is implementing the
backpropagation. The output of backpropagation is in effect just a $\theta$
type, so what we need to compute backpropagation is an input value, the actual
output value, the network, and we'll return a $\theta$.

Similarly to how `foldl` rather cleanly describes the forward pass of the neural
network. We can use `scanr`, which does effectively the same thing, only instead
of from the left it does so from the right, and also instead of returning the
accumulated value, it returns a list of iterations, which, for a recursive
formula like the one that the backpropagation equations describe, is a perfect
fit. All told, this is what a backpropagation implementation looks like:

```Haskell
backprop :: Vector Double -> Vector Double -> Network -> Theta
backprop x y (Network (Theta ts) c' f f') =
    let
        (y_hat, zsD) = runWriter (forwardW (Network (Theta ts) c' f f') x)
        as = DL.toList zsD -- [a1, a2 .., aL]
        activations = init (x : as) -- [a0, a1, a2 ... a(L-1)]
        derivatives = map (cmap f') (init as) -- [f'(a1), f'(a2)..., f'(aL-1)]
        deltaL = c' y_hat y
        deltas = scanr go deltaL (zip (tail ts) derivatives) -- [((w2, b2), f'(a1))...]
        go :: ((Matrix Double, Vector Double), Vector Double) -> Vector Double -> Vector Double
        go ((w, _), z) d = (tr w #> d) * z
        gradw = zipWith (\a d -> asColumn d <> asRow a) activations deltas
     in
        Theta (zip gradw deltas)
```

## Training montage time

Finally, all that's left to do is get some data to test on. We can generate some
of the sin function pretty easily:

```Haskell
sinData :: (RandomGen g) => Int -> g -> [(Vector Double, Vector Double)]
sinData k gen = zip x y
  where
    x = fmap scalar $ take k $ uniformRs (0 :: Double, 2 * pi :: Double) gen
    y = fmap sin x

sinTest :: [(Double, Double)]
sinTest = zip x y
  where
    x = [0, 0.01 .. 2 * pi]
    y = map sin x
```

The `$` operator is the lowest priority, right associative infix operator, it
simply takes `a` on the left and applies it to `b` on the right. This is in
contrast to normal function application, which is the highest priority and left
associative. In particle terms, it dictates that `take k` is applied to the
result of `uniformRs` before `fmap scalar` is ever applied. The above `x`
binding can be equivalently rewritten as

```{Haskell}
x = fmap scalar (take k (uniformRs (0 :: Double, 2 * pi :: Double) gen))
```

I hope you can see why `$` makes life a little easier. Before we move on to the
main function, I'd like to highlight a neat feature of Haskell, namely, how a
feature called lazy evaluation allows us to generate arbitrary random numbers.
Lazy evaluation dictates that code will only be ran when it is needed, this
allows lists to be practically "infinite". The function `uniformRs` takes a
range, in this case from $0$ to $2\pi$, and a generator, and returns an
arbitrarily long list of random numbers, uniformly distributed in that range.
When we `take k`, we're basically asking Haskell to evaluate the first `k`, and
return it as a list, this is very similar to how we can treat iterators as
infinite lists in python. I find lazy evaluation to be one of those language
features that, due to the reality of hardware, is understandably missing from
mainstream programming paradigms (although, I guess JIT is in its own way lazily
evaluating programs), but is an incredibly natural way to reason about your
program.

We can put all of the above together in a main function like so:

```Haskell
main :: IO ()
main = do
    let
        gen = mkStdGen 10
        dims = Dimensions 1 1 10 10
        theta = initNetwork dims gen
        net = Network theta gradEuclidean relu relu'
        trained = scanl train net (replicate 100 $ scaleInput <$> sinData 30000 gen)
        vectorTestList = map (scalar . fst) $ scaleInput <$> sinTest
        tests = map (\n -> map (fst . runWriter . forwardW n) vectorTestList) trained
        inputResultList = map (zip ([0, 0.01 .. 2 * pi] :: [Double])) (fmap (map (! 0)) tests)
        scaleInput (input, test) = (input / (2 * pi), test)
    writeFile "output.txt" $ show inputResultList
```

This funny looking operator `<$>` is the infixed version of `fmap` is really
just saying, "take the function on the left and apply it to the list on the
right". The `.` operator is the infixed composition operator. I didn't include
it since it's rather trivial but `scaleInput` simply normalizes the data input
to range from $[0,1]$, which is mostly standard in machine learning since
networks can be a little fussy about the magnitude of their inputs. We then scan
over all of the training batches, keeping a record of the network at that point,
and then we test it and extract the output by `fst . runWriter . forwardW`.
Finally, we write the array to a file, which is rendered by plotly at the top of
the page.

# Conclusion

## A Few Caveats

So far, I've been using the `hmatrix`'s dynamic API. That is, the matrices and
vectors that we compute on are determined at run time, as opposed to the static
API, where the matrices and vectors are of fixed length determined at
compilation time. While this works, it's problematic for three reasons:

- We pay a computational overhead that would not be there if we used static
  objects
- We aren't actually using the dynamic aspects of the objects in any meaningful
  manner. We aren't ever resizing the matrices or vectors in runtime, and in
  general, that's not something any ML framework would support.
- Network dimensions and inputs can be incorrectly constructed, with exceptions
  only being caught in runtime

One of the cool things about Haskell is that the type system allows you to
create programs which make invalid states impossible to represent: if my network
takes three inputs, I should never allow you to call it with a 4 element vector.
So we should use `hmatrix`'s static API, which bakes the dimensions of the
vectors and matrices into their type information. As of writing, I ended up
doing this but did not include it in this article because it required quite a
bit of type trickery, namely, keeping things in a list is difficult if they're
all of different types, and if we're including the dimensions in a type, each
layer is a different type. The way around this is using some pretty advanced
Haskell language features like `FunctionalDependencies` and `GADTs` which allow
us to construct non-homogeneous lists.

## Wrapping Up

At the top of the page you can view the fruits of our labour. We pretty closely
manages to converge to the desired sin function. So, whats next? There's a lot
of even the basics of neural networks we haven't touched on yet. It would be
interesting to implement the various optimizers that are industry standards now,
namely AdamW and Muon. Muon especially has roots in polynomial algebra, and I
wonder if Haskell will have an elegant representation of it. Once the basics are
covered, I think I'll try my hand at implementing more advanced architectures,
firstly CNN, and once that's done the sky's the limit really. I also want to
refactor the way I'm doing the forward pass to be a bit more modular in nature,
kind of like PyTorch where a network is composed of several separate layers, a
design decision that should have, honestly, been obvious considering that
composition is one of the monads strengths. Lastly, towards the end of this
project the training times did get a little prohibitive, and I think it would be
really interesting to see how well Haskell can handle parallelizing code;
alternatively, using a library like `copilot` which has been making waves in the
embedded space for the last couple of years, we can pawn out the computationally
expensive subroutines to CUDA code and only worry about high level architecture
in Haskell.
