Statisticians and probability theorists have historically salvaged and glued
together from various mathematical disciplines to form their
study. Measure spaces and $\sigma$-algebras borrowed from topology,
characteristic functions were appropriated from Fourier analysis, entropy
got nicked from thermodynamics; first by information theory then from
there by statistics. The list goes on, and we haven't even mentioned
geometry, linear algebra, or number theory, the latter two comprise
a decent chunk of introductory statistics. There's maybe an interesting
thesis-sized article tracking the genealogy of this frankly pieced
together field. 

While the ability for the various disciplines of mathematics to accommodate
one another so well is one of maths more beautiful features, and the
statistician's eagerness to knead together several often unrelated
ingredients to make the $\pi$ is often their virtue, it does make the
subject incredibly hard to teach. There's a lot that is simply taken for
granted, and the rigorous construction of the daily tools is skipped
because of the amount of content that there is to cover. As a result,
I find that, at least at the undergrad level, most statistics students
struggle to intuitively explain a tool like the $t$-distribution. And
that's statistics students; biologists, economists, and psychologist who
need the tools but often lack the prerequisite mathematics knowledge will
often treat statistics with learned helplessness. Of course
anecdotal, don't worry, I don't go around interrogating people about their
statistical intuition. The hope of this article is to provide some
mathematical meaning to the procedure that is often stated and briefly
glossed over.

Let's start with the problem statement, given a collection of data points
(referred to as the sample), drawn from some unknown population, can we
make any claims about the *characteristics* of the population, from the
*statistics* of the sample? As it turns out, yeah (kinda). It's important
to recognize here that each sample $x_i$ is a realization of a random
variable $X$; our population is *not* random, however, it is exactly
fixed. This is an important distinction to keep in mind. We use the
framing of probability to give us the language to quantify what we don't
know about our population.

Let's pick out a characteristic of our population that we're interested in
and figure out what our sample tells us about it. For example, what can we
say about the mean of our population, denoted as $\mu$. A natural place to
start if we want to figure out what $\mu$ is would be to calculate the
*sample* mean, let's denote it as $\bar x$ (another result of the variety
of statistics is that we still haven't really come up with good notation).
Naively, we can say $$ \mu \approx \bar x$$ And the closer our sample gets
to our population, the more likely this is to be. But collecting data is
difficult and expensive, and we'd like something a bit more rigorous than
$\approx$, exactly *how likely* is it for $\mu = \bar x$?. 

Since $x_i$ is a realization of a random variable $X_i$, and a random
variable scaled by a constant as well as the summation of random variables
is also a random variable, $\bar x$ is the realization of a random
variable as well, $$\bar X = \frac1n\sum X_i$$. To make life a bit easier,
lets assume that we know the distribution of $X_i$ and by extension of
$\bar X$. If in fact $\bar x = \mu$ then we would expect $\bar x-\mu
= 0 \implies E(\bar X)- \mu = 0$, in turn $$E(\bar X - \mu) = 0$$ 

We should pause here a little before continuing. What we said in the above
is that *if* $\bar x = \mu$ then the *distribution* of $\bar X - \mu$ will
have a mean of $0$. So, if we know what distribution $\bar X - \mu$
follows, than we can test if our *hypothesis* $\bar x = \mu$ is correct.
In fact, if we know the distribution of $\bar X - \mu$ we can evaluate
exactly how probable it is that $\bar x = \mu$. So... what is the
distribution of $\bar X - \mu$?

Well, if we dictate that our hypothesis is true, then we already know the
mean, what about the variance? Because variance is shift-invariant, that
is, adding or subtracting by a constrant does not effect the variance of
the random variable:

$$ \operatorname{Var}(\bar X - \mu) = \operatorname{Var}(\bar X)  $$ 

We can expand the averaging and do some algebra

$$ 
\begin{align*} 
\operatorname{Var}(\frac1n\sum X_i) &= \frac1{n^2}\operatorname{Var}(\sum X_i) \\
&= \frac1{n^2}[\operatorname{Var}(X_1) + \operatorname{Var}(X_2) + ... \operatorname{Var}(X_n) + \operatorname{Cov}(X_1, X_2) ...]
\end{align*}`
$$

In most cases where we apply the t-test, we like to assume that $X_i$'s
are indepdnent of each other, which means that $\operatorname{Cov}(X_i,
X_j) = 0, \forall i \ne j$. This is often a very reasonable assumption,
e.g. in a population of humans of the same age, heights can be safely
assumed to be indpendent. But we shouldn't get too complacent, indpendence
of samples is not always the case, the obvious example is time series
data, where each new event is influenced by the last, but even the example
of heights can be dubious, think of heights in a family, which is surely
not indpendent.


Anyway, since each $X_i$ is independent, it must be the case that their variances are the same, so we get:

$$ 
\begin{align*} 
\operatorname{Var}(\frac1n\sum X_i) &= \frac1{n^2}\sum{\operatorname{Var}(X_i)} \\
&= \frac1{n^2}\cdot n \operatorname{Var}(X) \\
&= \frac{\operatorname{Var}(X)}{n}
\end{align*}`
$$

You probably know that we call $\operatorname{Var}(X) = \sigma^2$. So finally

$$
\operatorname{Var}(\bar X - \mu) = \frac{\sigma^2}{n}
$$

We want our transformed distribution to be easy to work with, and dividing it by it's variance is a good way to do that, we'll go over what that gives us soon. 

$$
\begin{align*}
\frac{n}{\sigma^2}\operatorname{Var}(\bar X - \mu) &= 1 \\
\operatorname{Var}(\left (\sqrt{\frac{n}{\sigma^2}} \right)\bar X - \mu)  &= 1
\end{align*}
$$

Effectively, we have divided our distribution by the *standard error* of
$\bar X$. This does a couple of things: firstly, it standardizes the
distribution to have a variance of $1$, it also makes the distribution
*dimensionless*. When we say a distribution (or any value) is
dimensionless, we mean that it does not have any units attached to it;
they most commonly appear as a result of a division by a measurement of
the same units, leading to cancellation of the "dimensions" of the
measurements. These are confusing but critical to a lot of modern science,
especially physics, where they allow us to say things in a very general
sense, regardless of the particular characteristics of the system.
Removing the dimensions of our system does a similar thing, we can now
speak of our transformed distribution in absolute terms, if two data sets
, whose population is sampled from the same type of distribution, fulfill
the hypothesis that $\bar x = \mu$, then they have the exact same
transformed distribution!

However, this is not yet a t-distribution. If we assume that our data
$X_i$ is normal, than the above transformed distribution is *exactly*
a standard normal distribution. 
