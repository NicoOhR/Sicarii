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
things to say, and we'll try to keep this one short. Mostly, I hope to
motivate both the monad construction, as well as maybe explain why
category theory is how we choose to talk about things in this case. To do
this, I'll introduce the monoid and apply it to a programmatic intuition
for about as far as we can before we have to introduce the category, which
should land us at monad in a natural manner. Really, I'm trying to
communicate what made monads "click" for me.


## A Word on Higher Algebra

Even if you went to *higher education institution of choice* to study
mathematics you've probably only taken a single class in abstract algebra,
and even if you have, you've probably blocked it out of your mind as best
you can. The reason being, I think, that abstraction is just difficult,
and should usually be justified. While I by no means can speak for the
field, personally, the study of abstract algebra is motivated by the fact
that operations *induce* a structure on otherwise uncoordinanted objects,
let's try to analyze the structure of a computer program.

## Monoid in `Set`

In classical set based algebra, the monoid is a set $T$ equipped with
a binary operation $(\cdot)$ which returns the an element in the set and
a neutral $(1)$ element with respect to that operation. The monoid can
also be thought of as a relaxed version of the group, which dictates that
every element have an inverse element with resepct to the binary
operation. We choose the monoid for our analysis instead of the group
because many of the operations that we use to program are "lossy" by
nature, we cannot undo overwriting memory, for example, so enforcing
invertibility on our program seems unreasonable. 

We can use the monoid to study a somewhat simplified register machine (a
Turning machine could also be used, but I am slightly more comfortable
thinking about registers and I suspect most programmers are). The register
machine is composed of several registers with unique addresses which store
non-negative integers. 
