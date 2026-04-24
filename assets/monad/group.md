Again because of Cayely's theorem, all groups are permutation groups, that
is, every group describes how to order a list of things. Before we use
that, let's try a counting problem: How many ways can you arrange $n$
objects, with $q$ of them being non-unique?

When all elements of the set are unique, the number of permutations is counted by the factorial of $n$, $!n$. Intuitively, this is because, if you think about the number of choices you have when building the ordering, you start with $n$ choices, then for any one of those $n$ choices, you now have $n - 1$ choices for your next element, going on like that, you'll get $n(n-1)(n-2)...(2)1 = !n$ choices. Thinking about how youw would construct the object of a counting problem (in this case the ordering of the set) is a relatively common way of building an intution.  Notice that if one of our elements repeats, we would be double counting it. For example, if I asked you how many ways we can order the collection of letters "B", "B", and "R", the factorial would count out to $6$, but you could count out the options: 

* "B" "B" "R"
* "B" "R" "B"
* "R" "B" "B"

While this is relatively easy to count out in your head for $n = 3$ and $q
= 2$ ($3$), it gets prohibitive at $n = 5$ and on, and what if there are
multiple classes of objects which are non-unique?

Groups help us account for this double counting, we

## The G

Classically, a **group** $G$ is a set, equipped with an associative,
closed operation $(\cdot)$, the fact that it's closed means that for $g_1,
g_2 \in G \implies g_1 \cdot g_2 \in G$. With resepct to $(\cdot)$
operation, there is an identity element $i$, such that $\forall g \in G,
g \cdot i = g$, and each element in the set of $G$ has an inverse, that is
$\forall g \in G, \exists g^{-1}: g \cdot g^{-1} = i$. Because of Cayley's
theorem, which we will not be covering today, we almost always view the
operation $\cdot$ as a *composition* of the the elements of $G$ and we
call it the *product* of $G$. If I can insist that you take away anything
from this article is that a *product* is *compositional*. 

Groups naturally describe a great deal of things, practically any
collection of actions that can be undone and done one after the other is
described as a group. Cayley's theorem describes this relationship between
arbitrary groups and permutations groups. Cayleys is one of those widely
cited "beauty of math" theorems, and for good reason, the proof itself
"just works out" in the way you would want all mathematics to just work
out. The statement itself is relatively simple with some definitions: think of isomorphic as an
equality between groups, and the subgroup of a symmetric group being a permutation group.

>Every group is isomorphic to a subgroup of a symmetric group

For our intuition here, try to conceptualize of some simple computer
programs as permutations on types, then, by the above theorem we can
describe this subsection of programs as a group. The elements of the group
would be the operations we're allowed to do on the memory, in other words
the function or composition of functions which make up our program, and
the product operation would exactly be composition, that is, feeding the
result of our one function to the next. 

We have to restrict ourselves to simple programs because a lot of what we
do in our programs loses information. For example, after we overwrite
a block of memory, we have literally lost the information that was on that
block, similarily, what about random numbers? how can you reverse I/O?
This is a problem because some of the elements of our group *don't have an
inverse*. Therefore, we have to relax our condition a little bit, we'll
maintain that there should be an identity function, any composition of
programs should still be in the set of all programs, but we should admit
functions which don't have an inverse. A set equiped with a closed,
associative operation, an identity, but with no requirement for every
element to have an inverse element, is called a monoid.

# Monoids

So, monoids express composition with just enough power to be useful but
without the overbearing restrictions of the groups: Running a program is
a composition of functions which transform state, this composition makes
transformers into monoids. The problem is that the type of functions, the
elements of our monoid, is still very heavily restricted by the
requirements of set theory, writ large. When we operate on sets, like we
do in practically every math class that is accessible to an undergrad, we
want our morphisms (arrows, functions) to be "pure". That is, for every
input there is exactly one output. While a great deal of the functions
that we write in code are in fact pure, any function which also operates
on or with the state of the world that our program lives in will make our
function unpure. What we need is to create move away from the arrows in
Set, and start thinking about the category that describes our programs
more accuratly.

That is, a program $(\to)$ which takes an input of type $A$, and outputs
a type $B$, $(f: A \to B)$, but also which interacts with the world, in
other words, state of a program, must also include in it a description of
the effects of that computation, which we express as the following:

$$f: A \to T(B) $$

$T$ is what is called a type constructor, but morally we read out the
above as "the function $f$ sends an input in $A$ to a Transformed $B$". We
now have the language required to speak about effectful functions, but
we've introduced a new problem, we can no longer compose functions quite
as nicely as we would like. That is, if we have: 

$$f: A \to T(B) $$ and $$g: B \to T(C) $$

We cannot compose $f \cdot g$ because the types simply do not line up,
it's ill typed. The solution to this problem is to explicitly change the
the nature of our composition, which I'll denote as $\cdot_K$ for reasons
that will become obvious in a second. This notion of composition gives us
the ability to compsoe together effectful transformations, but it's
specific implementation is dependent on the transformation, which should
become clearer when get to concrete implementations.


<!-- I won't be going too deep into the definitions of categories because, -->
<!-- frankly, I don't think I can. With the assumption that you, reader, are -->
<!-- a software persom (tm), and that you probably won't spend too much more -->
<!-- time on category theory (read, abstract nonesense), I think we'll be fine -->
<!-- going ahead with the plain english interpretation of the word category, -->
<!-- that is, "a class of objects sharing some characteristics". Importantly, -->
<!-- for the authentic categorical experience, we need to care more about the -->
<!-- relationship between objects (arrows, morphisms) than what's "inside" the -->
<!-- object.  -->
<!---->
<!-- When we say "a Kleisli category" we're describing a construction on top of -->
<!-- another category, one which has the same objects, and which has the same  -->
