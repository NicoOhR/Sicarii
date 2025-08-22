This will be a very brief overview of our 2024 HackUTD hackathon project.
There were a couple of very interesting sponsor prompts, and I'm still
a little sad we never got the chance to use Pinata. We went
with PNC's prompt, to build tooling for observability and data lifetimes.
Of course, the word "observability" and "lifetimes" immediately caused us
to think of Kafka and message queueing. So, for our project, we decided to
build a platform to model and display the end to end life cycle of data
cleanly and efficiently. To that end, with only one all-nighter pulled. We
produced ``Sauron`` (a very clever name I came up with my self, thank you
very much). [Here is a link of
a demo](https://www.youtube.com/watch?v=SM_0w7rKo2c) that due to time
constraints we had to make before we could fully hook it up the Confluents
instance that we were simulating our Extract, Transform, Load pipeline on.
[And here is the GitHub repo](https://github.com/NicoOhR/sauron.git) 

Briefly, we used Terraform to spin up a Confluents instance, as well as
several small python cli programs (which were containerized and
orchestrated over k8) which acted as modular consumer-producers to
simulate micro-services in our pipeline. We utilized some minimal (and
very under documented) OpenTelemetry python instrumentation, written
directly into the service, which hooked up to a ```Jaeger``` instance
(also deployed on k8). Finally, we somehow (I was asleep) relayed the
Jaeger data into our front end, which you could see in the video.

Overall it was a very good, although exhausting hackathon, and we produced
a very interesting project. I learned a lot, and was definitely the most
hands on experience that I've had with Kafka. A couple take aways:

* Maybe python isn't the best language for a hackathon. Go is synonymous
with cloud native architecture, and it really felt like Jaeger and really
the entire stack only *tolerated* python. 
* Tooling, as opposed to consumer software, is difficult to build
partially because the standards and protocols which are used have "rougher
edges". This is very similar to my experience working in embedded spaces,
where protocols designed to I/O with a non-technical user (e.g. USB, SCPI)
are much easier to leverage than whatever is going on with QUADSPI.
