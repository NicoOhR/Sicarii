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
say about the mean of our population, denoted as $\mu$. What we want to do
is take the information we got from the sample mean, denoted as $\bar x$,
and use it to test a guess (read, hypothesis) for $\mu$, which we denote
as $\mu_0$. Basically we want to ask, *how likely is it that
a distribution that produced $\bar x$ would have the actual mean $\mu_0$

Since $x_i$ is a realization of a random variable $X_i$, and a random
variable scaled by a constant as well as the summation of random variables
is also a random variable, $\bar x$ is the realization of a random
variable as well, $$\bar X = \frac1n\sum X_i$$. To make life a bit easier,
lets assume that we know the distribution of $X_i$ and by extension of
$\bar X$. If in fact $\mu = \mu_0$ then we would expect that over the long
run, as the number of data points in our sample increases, then the
difference between $\bar X$ and $\mu_0$ decrease. Notionally, this
sentiment is expressed as the expected value: $$E(\bar X - \mu_0) = 0$$ 

We should pause here a little before continuing. What we said in the above
is that *if* $\mu = \mu_0$ then the *distribution* of $\bar X - \mu$ will
have a mean of $0$. So, if we know what distribution $\bar X - \mu_0$
follows, than we can test if our *hypothesis* $\mu = \mu_0$ is correct. In
fact, if we know the distribution of $\bar X - \mu_0$ we can evaluate
exactly how probable it is that $\mu = \mu_0$. So... what is the
distribution of $\bar X - \mu_0$?

Well, if we dictate that our hypothesis is true, then we already know the
mean, what about the variance? Because variance is shift-invariant, that
is, adding or subtracting by a constrant does not effect the variance of
the random variable:

$$ \operatorname{Var}(\bar X - \mu_0) = \operatorname{Var}(\bar X)  $$ 

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

Anyway, since each $X_i$ is independent, it must be the case that their
variances are the same, so we get:

$$ 
\begin{align*} 
\operatorname{Var}(\frac1n\sum X_i) &= \frac1{n^2}\sum{\operatorname{Var}(X_i)} \\
&= \frac1{n^2}\cdot n \operatorname{Var}(X) \\
&= \frac{\operatorname{Var}(X)}{n}
\end{align*}`
$$

You probably know that we call $\operatorname{Var}(X) = \sigma^2$. So finally

$$ \operatorname{Var}(\bar X - \mu_0) = \frac{\sigma^2}{n} $$

We want our transformed distribution to be easy to work with, and dividing
it by it's variance is a good way to do that, we'll go over what that
gives us soon. 

$$ \begin{align*} \frac{n}{\sigma^2}\operatorname{Var}(\bar X - \mu_0) &=
1 \\ \operatorname{Var}\left(\left (\sqrt{\frac{n}{\sigma^2}} \right)\bar
X - \mu_0\right) &= 1 \end{align*} $$ 

So we get our final distribution, which we'll call $T$: 

$$ T = \frac{\bar X - \mu_0}{\sqrt{\sigma^2/n}}$$

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
the hypothesis that $\mu = \mu_0$, then they have the exact same
transformed distribution! Historically, back when computers were not
common place and calculating the CDF of a particular distribution was time
consuming, having a common distriubtion that we can reference precomputed
values of was very important.

However, this is not yet a t-distribution. If we assume that our data
$X_i$ is normal, than the above transformed distribution is *exactly*
a standard normal distribution, which we often denote with a $Z$. The
t-distribution arises when we *estimate* the variance of the population.
It is much more common that we do not know the variance of our population,
*a priori*. So, how should we estimate the variance of our population?
A natural answer would be the variation of the sample.

$$ s^2 = \frac{1}{n-1}\sum (X_i - \bar X)^2$$

Again, we should recognize that $s^2$ is a random variable, which somewhat
complicates our transformed distribution. But have no fear, $s^2$ luckily
has a predictable distribution (after a little bit of fanengaling), we get
the $\chi^2$ distribution (pronounced cai-squared). The $\chi^2$
distribution arises as a sum of squared $N(0,1)$ distributions. The mean
of $(X_i - \bar X)^2$ is $0$ under our assumption that $\mu = \mu_0$, then
we scale it again by dividing by $\sigma^2$, finally we move the $n -1$ to
the other side so: 

$$ \frac{(n-1)s^2}{\sigma^2} = \frac{1}{\sigma^2}\sum (X_i - \bar X)^2
\sim \chi^2_{n-1}$$

So lets swap out $\sigma^2$ for $s^2$ in our transformed variable

$$ \frac{\bar X - \bar x}{s/\sqrt{n}} $$

To get the denominator to look like the $\chi^2$ distribution we
described, we need to do some pretty meaningless algebra, which I have
added for completness sake 

$$
\begin{align*}
T
  &= \frac{\bar X - \mu}{\,s/\sqrt{n}\,} \\[4pt] &= \frac{(\bar
  X - \mu)\,(\sqrt{n}/\sigma)}{(s/\sqrt{n})\,(\sqrt{n}/\sigma)} \\ &=
  \frac{\displaystyle \frac{\bar X - \mu}{\sigma/\sqrt{n}}}{\displaystyle
  s/\sigma} \\[10pt] &= \frac{Z}{\,s/\sigma\,} \\[8pt] &=
  \frac{Z}{\sqrt{s^2/\sigma^2}} \\[6pt] &=
  \frac{Z}{\sqrt{\frac{(n-1)s^2/\sigma^2}{\,n-1\,}}}  \\ &=
  \frac{Z}{\sqrt{V/n-1}}. \end{align*} $$

So we get that $T$ is the ratio of the standard normal distribution the
square root of a scaled $\chi^2$ distribution, which, finally, is the
t-distribution, denoted by $t$. A complete proof of this fact can be found on one of my
[favorite sites](https://statproofbook.github.io/P/t-pdf.html).

Phew. That was a lot of work. So... why's any of this useful? This whole
derviation hinges on the fact that $\mu = \mu_0$ that is

$$ T = \frac{\bar X - \mu_0}{s/\sqrt{n}} \sim t $$

*Only under the null hypothesis* $H_0: \mu = \mu_0$. So, if we plug in the
actually observed values $\bar x$ for $\bar X$, we can see how extreme
(how unlikely) it is that, in fact $\mu = \mu_0$ is to be. The more
extreme of a value our realization of $T$ is, the less likely it is that
our hypothesis is correct.
