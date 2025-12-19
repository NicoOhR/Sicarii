{{{assets/haskellNN/training.html}}}

Every year, starting at December 1st, a new programming puzzle is
published to adventofcode.com. Each puzzle contains in it two parts, and
are almost always some of the best designed puzzles I play during the
year. I typically solve Advent of Code in Haskell, since I find that the
strictly functional nature of the language encourages more interesting
solutions than the brute force calculations that are often possible. The
Haskell itch came a little early this year and advent isn't starting for another... sorry? it's already December? Look I already wrote this intro and the scope of this project crept a little bit, so let's just agree to move on my inability to keep to self imposed deadlines yeah?
Anyway, it didn't exactly sit right with me that I have never implemented a neural
network from scratch by myself, and the fact that Haskell is pretty
awesome for linear algebra stuff, as we will see, I figured it would be
fun to go through the implementation of an NN in Haskell. This article
assumes no knowledge of Haskell or machine learning, but does require some
knowledge in calculus and linear algebra, and that you are familiar with *a* programming language. I hope to achieve two things in
this article, firstly I want to demonstrate that the basic primitive of
the machine learning field, the neural network, is really not that
complicated or hard to get your head around, and secondly I would like to
answer the decently common question "why use Haskell?".

## Setting Up

First, let's try to appreciate what it is exactly that we are building.
Put into a sentence, a neural network is just *"the composition of linear
transformations and non-linear functions"* and we typically tack on
*"which approximates a continuous function in $\mathbb{R}^n$ "* implicitly.
Let's try to tackle each part of this definition, I'll keep it brief, and
will try to add resources for anyone who wants to read more

* A *linear transformation* is any transformation which transforms objects
in a globally consistent manner. For our purposes, that means that how much
input $x$ is transformed by linear transformation $W$ does not depend on
$x$ at all. In addition, we can add another vector after the
transformation while maintaining linearity, for a reason that we'll get to
in a minute, this is very useful.
* A *non-linear function* is all other transformation, that is, non-linear
function $f$ transforms $x$ differently depending on the specific value of
$x$
* A *continuous function in* $\mathbb{R}^n$ is a vector valued function
where a small variation of the input will lead to a similarly small
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
this will probably be the only Greek letter we use) we can approximate any
function $f$, kinda. Quick mathematical aside: there is a theorem named,
quite badly, universal approximation theorem, which states that for any
continuous function $f$ and an arbitrary $\varepsilon > 0$, there exists
an arbitrarily large linear transformation $A$ and some $b$ such that for
all $x$, 

$$||f(x) - g(x)|| < \varepsilon$$

We're in the real world, so we might not be able to compute the "arbitrarily large" $A$, but this is rarely if ever a problem.

## Actually Finding the Parameters.

In optimization, we generally like thinking about minimization problems rather than maximization problems, more by convention than by any other reason. In this case we're trying to find the parameters which are *least wrong* which is the same as the parameters that are *most right*. This is expressed notationaly as 

$$
\operatorname*{argmin}_\theta ||f(x) - g\_{\theta}(x)||
$$

And is read as "find $theta$ such that the difference between the function $f$ and the the function paramterized by $\theta$, $g$, is minimized". In this case, we can use the standard euclidean norm

$$
||v|| = \sqrt{v_1^2 + v_2^2 + ... v_n^2}
$$

To compute the difference between our functions, but sometimes, depending on your problem and what the output of your network is, other *metrics* should be used (for example, for probability distributions, we use the KL-divergence metric to quantify how far our network is from our function). Because we'll do the chosen metric operation a lot, we'd like it to be computationally cheap, and the square root function is anything but. We can avoid doing the square root by recognizing that, because the square function is strictly increasing for positive values, minimizing the square of the norm is exactly the same as minimizing the norm itself, so, our goals becomes: 


$$
\operatorname*{argmin}_\theta~ ||f(x) - g(x)||^2
$$

Or more explicitly 

$$
\operatorname*{argmin}_\theta~ (f(x)_1 - g(x)_1)^2+(f(x)_2-g(x)_2)^2+...(f(x)_n - g(x)_n)^2
$$

We call the measure of how "wrong" our network is the cost function (also loss, depending on whose writing the paper) and we denote it by $C$. Cost is parameterized by both $\theta$ and $x$, which is important to keep in mind as we do the derivations for backpropagation, the algorithm of choice to find the best parameters.

Now that we've stated the goal, how do we find it? Here we introduce the gradient, which is, for a vector valued function $f: \mathbb{R}^n \to \mathbb{R}$ defined as 

$$
\nabla f(\mathbf{x}) =
\begin{pmatrix}
\frac{\partial f}{\partial x_1}(\mathbf{x}) \\
\frac{\partial f}{\partial x_2}(\mathbf{x}) \\
\vdots \\
\frac{\partial f}{\partial x_n}(\mathbf{x})
\end{pmatrix}
$$

I'll ask you to take the following on axiom, the gradient of $f$, $\nabla f$ evaluated at $x$ is the direction of fastest change positive change, and $-\nabla f$ is the direction of fastest negative change. When (read, if) I find the time to figure out javascript widgets, there will be a nice interactive diagram showing, geometrically, why this is the case. So if we follow the negative gradient of the cost function at every point, we'll eventually find *a* minimum! Importantly, this is not the global minimum, only a local minimum, where "moving" in any direction would increase the function.

So to find the optimal $\theta$, we find the the negative gradient of the cost function, with respect to $\theta$ for each $x$ in our data set, and move along the direction of the gradient. There are really quite a lot of addendums and technicalities that we're glossing over with this corse treatment. But we're working fast here, and a slightly more rigorous treatment which I suspect most people interested in machine learning have already read can be found [here](http://neuralnetworksanddeeplearning.com/index.html) , written by Michael Nielsen. Since I rather poorly timed this project with finals season, I will be skipping the derivation of backpropagation for the time being. Taken from the link above, the governing equations we need to calculate how much we need to change our parameters by is given as: 


$$
\begin{aligned}
\delta^L &= \nabla_a C \odot \sigma'(z^L) \\\\
\delta^l &= ((W^{l+1})^T\delta^{l+1}) \odot \sigma'(z^l) \\\\
\frac{\partial C}{\partial b^l_j} &= \delta^l_j \\\\
\frac{\partial C}{\partial w_{j,k}^l} &= a_l^{l-1}\delta_j^l
\end{aligned}
$$

Where $z^l$ is the output of the $l$-th layer prior to going through the activation function $\sigma$, and $a^l$ is the result of passing $z^l$ though the activation function.

## Haskell my sweet prince ( ˘ ³˘)♥

If you've never seen functional programming before some of this might be
an adjustment, but give me more time than its probably worth and I promise
you'll be more confused than when we started. Snark aside, one of my
favorite aspects of functional programming is that once you have a good
representation of your data, you've made some pretty decent progress
towards solving your problem.

So far, we have a non-linear function, which we call an *activation
function* and we have our parameters. 

```Haskell
data Theta = Theta [(Matrix Double, Vector Double)] deriving (Show)

type Activation = Double -> Double
``` 

`data` is how we define a new type in Haskell, the `Theta` on the
other side of the definition is the data constructor of this type. While
not strictly necessary, I like using data constructors as a sort of built
in label for parameters which is helpful if you're, like me, bad about
naming your variables. ```[(Matrix Double, Vector Double)]``` means that
this is a list of the tuple of a matrix of doubles, and a vector of
doubles, our $W^i$ and $b^i$ respectively. `deriving (Show)` tells the compiler that we want this type to be show-able, which in this case just means that it can be coerced into a string so we can print it. When we can or cannot derive a *typeclass* (what we call characteristics) is dependent on a couple of things, but `Show` is relatively simple I thing. If you've programmed in Rust before, this is largely where `#[derive(Debug)]` comes from. The `type` keyword defines a synonyms for a type, as opposed to `data` which defines a *new* type.  In this case, it's just nicer to give this type a name.


Eventually, we'll need to subtract thetas and scale them by a `double` type. Of course, this is exactly defining a vector space over the theta type, and we could probably define a vector space typeclass to get some nice binary operations to do this, but this is just as achievable, and probably easier, to just do with functions.

```Haskell
scaleThetas :: Theta -> Double -> Theta
scaleThetas (Theta ts) eta = Theta (fmap (\(x, y) -> (scale eta x, scale eta y)) ts)

subtractThetas :: Theta -> Theta -> Theta
subtractThetas (Theta ts) (Theta gs) = Theta (zipWith (\(x, y) (u, v) -> (x - u, y - v)) ts gs)
```

The double colon, `::` should be read as "has type of". Exactly why there isn't a distinction between what are procedurally thought of as the parameters and the output of the function is an iota outside of the scope of this article unfortunately, but the keyword to google here is *currying* (maybe like the food, I never asked). First, looking at the `scaleThetas` function, we take as input a theta and a double, then fmaps the anonymous function `(\x,y) -> (scale eta x, scale eta y)` on the `Theta`. `fmap` stands for "function map", and does just that, given a function and a list, return the list with the function applied to each element of the list, and `scale` is provided by the Vector and Matrix types to do scalar multiplication. Looking at the `subtractThetas` function, `zipWith` is a combination of `fmap` and `zip`, so we're combining two lists, the inputs `ts` and `gs`, then applying the lambda function `\(x,y) (u,v) -> (x - u, (y - v))` to that combined list. 
Let's write the function for running running our nueral network, that is, of obtaining an approximation $\hat y$ for an input $x$. Remember that our network is just a composition of matricies, vector additions, and activation functions which we apply iteratively, this is rather cleanly expressed by the `fold` function which haskell provides us with. 

Interpreted as concretly as possible, the `fold` function takes a binary operator, an accumulator and a list, applies the operator to each element in the list with the accumulator, in order. `foldl` "folds" the list starting from the left, and has a type of `foldl:: Foldable t => (b -> a -> b) -> b -> t a -> b`. `Foldable t` is a restriction on the type of `t`, meaning that whatever `t a` is, it should be foldable over, this is another example of a typeclass, like `Show` from earlier. In our case, we'll be iterating over our $\theta$ which has the list type, which is foldable over. Next, we see from the type that we want a function `(b -> a -> b)` that is, it takes a type `b` and a type `a` and gives back a type `b`, this is our binary operator. Then we also require a `b`, which is our accumulator value, and finaly the list `t a` which we process, returning a `b`. Maybe you can already see how this applies to our neural network. Recall our mathematical definition of a neural network: 

$$ g(x) = f(W^L(f(W^{L-1}f(...f(W^1x + b^1)))+b^{L-1})+b^L) $$

We want to end up with an output vector, so `b` is already determined to be of type `Vector Double`, and we want to process over our `Theta`, so `a` naturally becomes the tuple `(Matrix Double, Vector Double)`. If we keep our activation function $f$ consistent across all layers, the above formula is expressed rather cleanly as

```Haskell
forward :: Network -> Vector Double -> Vector Double
forward (Network (Theta ts) _ f _) x0 =
    foldl step x0 ts
  where
    step :: Vector Double -> (Matrix Double, Vector Double) -> Vector Double
    step x (w, b) = cmap f (w #> x + b)
```

`#>` is just the Matrix-by-vector multiplication that `hmatrix`, the linear algebra library I tend to default to, uses. As nice as this is, if you're familiar with neural networks you might be wrinkling your nose a little right now. Firstly, we'll often not apply an activation layer to our last layer to ensure that our network can produce the entire range of values we're interested in, so we'll need to special case our last layer. Additionally, recall from the backpropagation equations, we need *all* $z^l$, not just the final output. But never fret reader, any problem is really just a teachable moment, and we'll use this particular problem to learn about monads! 

## Monoids in the Category of Endofunctors

One of the more famously unhelpful sentences in programming is the answer to the question "What is a monad?" which, canonically, is "Well simple! It's just a monoid in the category of endofunctors! Of course!". Believe it or not, with only a little bit of abstract algebra and category theory, this is a relatively simple defintion, but feels almost impossible to ground to programming. There are countless blogs and videos (I reccomend the sheafification of g's video on the subject personally) trying to explain monads; it's a rite of passage of sorts I think. Today, we'll focus specifically on only the monad we care about, the `Writer` monad, to demonstrate how you would actually use one of these things in the field. I will qoute Kwang's excellent article on the monad to define it 

    "The Writer monad represents computations which produce a stream of data in addition to the computed values"

This definition is a little funny because, in many ways, this is what *all* monads do; encode additional "sideffects" of otherwise pure code. When in the context of the writer monad, we can use `tell` to add to our list of logs, and `pure` to give back our actual value. A slight hitch in this is that `foldl` is a pure function, which means that the type checker will get very mad at you if you try to use monads with it, luckily we have `foldlM` which allows us to pass monadic functions to it. Implementing the writer monad into our naive `forward` we get 

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

(Authors note: This is wrong now that I look at it, we should be `tell`ing the preacticated `z`s, with ReLU the bug is functionally invisable but does need to be fixed).

Every step of the `foldlM`, we `tell` to our List the resulting `z`, and pass to the next call `pure z`. The resulting output of the `fowardW` function is `[y_hat, [a_1, a_2, a_3 ... a_l]]` (`DList` is an interesting data structure that I will not be reviewing today, think of it simply as a list). The last piece of the puzzle before we can train our neural network is implementing the backpropagation. The output of backpropagation is in effect just a $\theta$ type, so what we need to compute backpropagation is an input value, the actual output value, the network, and we'll return a $\theta$.  

Similarily to how `foldl` rather cleanly desribes the forward pass of the neural network. We can use `scanr`, which does effectively the same thing, only instead of from the left it does so from the right, and also instead of returning the accumulated value, it returns a list of iterations, which, for a recursive formula like the one that the backpropagation equations describe, is a perfect fit. All told, this is what a backpropagation implementation looks like: 

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

Finally, all that's left to do is get some data to test on. We can generate some of the sin function pretty easily: 

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

and now we can run it like so: 

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

## Wrapping Up 

At the top of the page you can view the fruits of our labour. We pretty closely manage to converge to the desired sin function. So, whats next? Well, there's a lot of even the basics of neural networks we haven't touched on yet. It would be interesting, I think to implement the various optimizers that are industry standards now, namely AdamW and Muon are both on the list. Once the basics are covered, I think I'll try my hand at implementing more advanced architectures, firstly CNN, and once that's done the skys the limit really. I also want to refactor the way I'm doing the forward pass to be a bit more modular in nature, kind of like PyTorch where a network is composed of several seperate layers, a design desicision that should have, honestly, been rather obvious considering that composition is one of the monads strengths. Lastly, towards the end of this project the training times did get a little prohibitive, and I think it would be really interesting to see how well Haskell can handle parallelizing code. 
