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

## Setting up

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

The step above is critical to our understanding of this subject, so we
should pause here a little before continuing. What we said in the above is
that *if* $\mu = \mu_0$ then the *distribution* of $\bar X - \mu$ will
have a mean of $0$. This works because $\bar X$ is what we call an
*unbiased estimator* of $\mu$. An estimator (which is a random variable)
is unbiased if it's *expected value* is equal to the true value. There's
a couple of ways to think of the expected value function $E$, but it's
probably easiest to conceptualize it as the *average* of realizations of
a random variable, *weighted* by the likelihood of them appearing.
Mathematically this is expressed as either a summation (in the discrete
random variable case), or integration (in the continuous random variable
case). So

$$
\begin{align*}
\operatorname{E}[\bar X] = \mu &\implies \operatorname{E}[\bar X] - \mu_0 = \mu - \mu_0  \\
&\implies \operatorname{E}[\bar X - \mu_0] = \mu - \mu_0 = 0
\end{align*}
$$

Philosophically, applying the expected value operation allows us to stop
thinking about specific realizations of $\bar X$, and start thinking about
the overall *expectation* of $\bar X$. So, if we know what distribution
$\bar X - \mu_0$ follows, than we can test if our *hypothesis* $\mu
= \mu_0$ is correct. In fact, if we know the distribution of $\bar
X - \mu_0$ we can evaluate exactly how probable it is that $\mu = \mu_0$.
So... what is the distribution of $\bar X - \mu_0$?

Well, if we dictate that our hypothesis is true, then we already know the
mean, what about the variance? Because variance is shift-invariant, that
is, adding or subtracting by a constant does not effect the variance of
the random variable:

$$ \operatorname{Var}(\bar X - \mu_0) = \operatorname{Var}(\bar X)  $$ 

We can expand the averaging and do some algebra

$$ 
\begin{align*} 
\operatorname{Var}(\frac1n\sum X_i) &= \frac1{n^2}\operatorname{Var}(\sum X_i) \\
&= \frac1{n^2}[\operatorname{Var}(X_1) + \operatorname{Var}(X_2) + ... \operatorname{Var}(X_n) + \operatorname{Cov}(X_1, X_2) ... + \operatorname{Cov}(X_{n-1}, X_{n})]
\end{align*}
$$

In most cases where we apply the t-test, we like to assume that $X_i$'s
are independent of each other, which means that $\operatorname{Cov}(X_i,
X_j) = 0, \forall i \ne j$. This is often a very reasonable assumption,
e.g. in a population of humans of the same age, heights can be safely
assumed to be independent. But we shouldn't get too complacent, independent
of samples is not always the case, the obvious example is time series
data, where each new event is influenced by the last, but even the example
of heights can be dubious, think of heights in a family, which is surely
not independent.

Anyway, we also assume that each $X_i$ is identical. A collection of
random variables that identical and independently distributed will often
be abbreviates as $\text{i.i.d}$. Because they are identical, they share
a variance.

$$ 
\begin{align*} 
\operatorname{Var}(\frac1n\sum X_i) &= \frac1{n^2}\sum{\operatorname{Var}(X_i)} \\
&= \frac1{n^2}\cdot n \operatorname{Var}(X) \\
&= \frac{\operatorname{Var}(X)}{n}
\end{align*}
$$

You probably know that we call $\operatorname{Var}(X) = \sigma^2$. So finally

$$ \operatorname{Var}(\bar X - \mu_0) = \frac{\sigma^2}{n} $$

(The division by $n$ has additional meaning beyond being a result of algebra, $n$ being the *degrees of freedom* of the data. This is a subject deserving it's own article, and if you would like more information I can recommend Sam Levy's still ongoing [series](https://www.youtube.com/watch?v=VDlnuO96p58) on the subject).

We want our transformed distribution to be easy to work with, and dividing it by it's variance is a good way to do that, we'll go over what that gives us soon. 

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
consuming, having a common distribution that we can reference precomputed
values of was very important.

## the t-distribution

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
the $\chi^2_{n}$ distribution (pronounced cai-squared with $n$ degrees of freedom). The $\chi^2$
distribution arises as a sum of $n$ squared $N(0,1)$. The mean
of $(X_i - \bar X)^2$ is $0$ under our assumption that $\mu = \mu_0$, then
we scale it again by dividing by $\sigma^2$, finally we move the $n -1$ to
the other side so: 

$$ \frac{(n-1)s^2}{\sigma^2} = \frac{1}{\sigma^2}\sum_{i=1}^n (X_i - \bar X)^2
\sim \chi^2_{n-1}$$

So lets swap out $\sigma^2$ for $s^2$ in our transformed variable

$$ \frac{\bar X - \bar x}{s/\sqrt{n}} $$

To get the denominator to look like the $\chi^2$ distribution we
described, we need to do some pretty meaningless algebra, which I have
added for completeness sake 

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

Where $V \sim \chi^2_{n-1}$. So we get that $T$ is the ratio of the standard normal distribution the
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
our hypothesis is correct. Finally, we have arrived at a way evaluate
a hypothesis in a numerical manner. 

## Some catching up

The main take away here is that under certain assumptions, namely:

1. The data is independent 

2. The data is roughly normally distributed

Then we can evaluate how likely it is that a hypothesized mean is in fact the true mean of our data (in reality, the t-test is empirically quite robust to non-normality with higher sample sizes). We did this by constructing a distribution (the $t$ distribution) which under the above assumptions and an additional hypothesis (that our guessed mean is equal to the true mean $H_0: \mu_0 = \mu$) will behave in a predictable way. Then, after calculating the observed value of the distribution (referred to as the $t$ value) we can check exactly how likely we were to see this value under the null hypothesis. 

We haven't actually discussed how to do the last step, we do this by using the $p$-value. I'll take a couple sentences here to emphasize something, it is *remarkably* easy to abuse the $p$-value. It's meaning is not directly obvious or intuitive, and the way we use the $p$-value makes it seem like a measure of veracity. This is not the case. What the $p$-value tells us is how likely it is that a distribution will take a value more extreme than the test statistic (the general term for something like the $t$ value) is, given the hypothesis. In formula form
$$
p = \text{Pr}[T\text{ is more extreme than }t\lvert H_0]
$$
I'm obfuscating a little bit because there are several ways to think of "extreme" in this scenario. For simplicity we'll work with the one sided right tail test statistic, but once we understand this it is not a large leap to understand the other

$$
p = \text{Pr}[T \ge t \lvert H_0]
$$

In the scenario that we've been working, where we want to test if our guess $\mu_0$ is close to $\mu$, a low $p$ value would mean that the probability that the $t$-distribution would produce the observed $t$ value low, which in turn means that we should reject the hypothesis that $\mu_0 = \mu$. In practice, we usually use the $t$ test the other way around; in the linear regression case, our null hypothesis is "this independent variable has no statistically significant impact on the regressed variable", that is, the coefficient of this independent variable in the linear regression is $0$. We test this by constructing the $t$ distribution in a basically identical way, now a low $p$ value would imply that it is very unlikely that the independent variable is irrelevant (again, this does not make any claim about the magnitude of this variable's impact, only that it is statistically relevant).

## Closing Remarks

I had originally planned to talk about some of the other tools that the $t$ distribution gives us namely confidence intervals, prediction intervals and touching on the basics of the ANOVA, although I already knew it deserved its own article; but I have severely underestimated how bad I am at being concise.
