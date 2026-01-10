## Do we really need another article about this?

In a previous article on implementing a neural network in Haskell we used
the writer monad to propagate intermediate calculations through the
forward pass of the network. While I wanted to explain the monad in its
full generality, I felt as though this subject has been covered to death
in the past by people significantly smarter than me, best illustrated by
this frankly hilarious graphic from the Haskell wiki:

<div style="text-align: center;">

![monads over time](/img/Monad-tutorials-chart.png) 

</div>

However, after finishing that article up, I found that I do have a few
things to say, and we'll try to keep this one short. In a couple hundered
words I hope to explain what monads are from both the mathematical
perspective (Eilenberg-Moore category) and the programming perspective
(Klesili categoties).

## A Word on Higher Algebra

Even if you went to *higher education institution of choice* to study
mathematics you've probably only taken a single class in abstract algebra,
and even if you have, you've probably blocked it out of your mind as best
you can. The reason being, I think, that abstraction is just difficult,
and should usually be justified. While I by no means can speak for the
field, personally, the study of abstract algebra is motivated by the fact
that operations *induce* a structure on otherwise uncoordinanted objects,
and understanding the structure they induce will organize objects into
certain high level categories. Going off of that, we'll now introduce (or
reintroduce) the concept of a *group*, explain how it explains the
structure of *actual* things, and off that we'll discuss the *monoid*
which will naturally lead us to monads.

## The G

Classically, a **group** $G$ is a set, equipped with an associative
operation $(\cdot)$, with resepct to that operation, there is an identity
element $i$, such that $\forall g \in G, g \cdot i = g$, and each element
in the set of $G$ has an inverse, that is $\forall g \in G, \exists
g^{-1}: g \cdot g^{-1} = i$. Because of Cayley's theorem, which we will
not be covering today, we almost always view the operation $\cdot$ as
a *composition* of the the elements of $G$ and we call it the *product* of
$G$. If I can insist that you take away anything from this article is that
a *product* is *compositional*. 

Again because of Cayely's theorem, all groups are permutation groups, that
is, every group describes how to order a list of things. Before we use
that, let's try a counting problem: How many ways can you arrange $n$
objects, with $q$ of them being non-unique?

While this is relatively easy to count out in your head for $n = 3$ and $q
= 2$ ($3$), it gets prohibitive at $n = 5$ and on, and what if there are
multiple classes of objecst which are non-unique?
