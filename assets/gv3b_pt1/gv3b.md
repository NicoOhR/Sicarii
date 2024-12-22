The three body problem is a relatively well known scenario in classical
mechanics, as well as a pretty good series of novels, with no closed form
solution. It's comprised of three, equal mass, bodies in space (the "three
body" part) and the aim is describing the movement caused by gravitational
attraction (the "problem" part). Some restricted variations of the problem have
solutions, and it's most generally described as a system of three second order
vector differential equations, or a really ugly eighteen part Hamiltonian
differential scalar equations. 

With all that said, since we're in the business of intellectual exercises,
and less so in the business of a particularly nasty Diff. Eq. homework
question, I set out to build a simulation of the three body problem. Well,
the original goal was, and time permitting, still is, to write a model to
seek out stable (rather, least chaotic) configurations of the scenario.
This, then, is part one of the project, setting up the simulation and
program environment.

The simulation is set up in Rust, using Bevy + Rapier 2D. I hesitated to commit
to using Bevy since I thought it would be overkill for what I needed it for,
and that was somewhat true, but this [particularly
interesting](https://www.youtube.com/watch?v=PND2Wpy6U-E) podcast convinced me
to bite the bullet and figure out Bevy's ECS system. With a little bit of
hindsight, I can say confidently it would have been easier to use Rapier as
a headless physics solver and simply visualize using matplotlib or similar down
the pipeline. Since the goal of the project is to write a model to solve our
scenario, I needed a way for the program to communicate with external program.
The choice to use an external program was motivated mostly because [we are not
learning yet](https://www.arewelearningyet.com/) and I also just wanted to play
around with interprocess communication. I chose to use gRPC by way of Tokio's
Tonic because I've been meaning to learn how to use gRPC for a while, but also
because of my fondness for the Tokio ecosystem, *in no small part due to their
neat-looking designs*.  

To be clear, graphQL and some form of REST would have probably done just
as well, if not better. At the projects current state, GraphQL would have
been a little more ergonomic on merit of it's JSON representation format.
If I wanted to, say, pass the simulation to another service, like
a diagnostics service, or some HPC platform, gRPC would have worked
wonders.

Now, onto the code. Let's start with the initial conditions, the program
looks in a file called ``config.toml`` which looks, for example, like
this:

```Toml
[[body_attributes]]
id = 1
radius = 40.0
restitution = 1.0
mass = 300.0
velocity = { x = 0.0, y = 25.0 }
position = { x = 500.0, y = 0.0 }

[[body_attributes]]
id = 2
radius = 40.0
restitution = 1.0
mass = 300.0
velocity = { x = 0.0, y = 25.0 }
position = { x = -500.0, y = 0.0 }

[[body_attributes]]
id = 3
radius = 40.0
restitution = 1.0
mass = 600.0
velocity = { x = 0.0, y = 0.0 }
position = { x = 0.0, y = 0.0 }
```

Which defines all the values rapier needs to initialize our rigid bodies.
I opted to use an external configuration file so that we can change the setup
of the bodies in runtime, without recompiling the program. Once our bodies our
spawned, we create a system called ``gravity_update()`` which is tad long, but
queries our bodies (using Bevy's ECS), gets the unique combinations of bodies
using the built in ``iter_combinations_mut::<2>()`` (practically the n choose
k combinatorial) and applys the force calculated by the
``gravitational_force()`` function

```Rust
pub fn gravitational_force(
    mass1: f32,
    mass2: f32,
    position1: Vector2<f32>,
    position2: Vector2<f32>,
) -> Vector2<f32> {
    let r = position2 - position1;
    let direction = r.norm();
    let f_mag = 1000.0 * ((mass1 * mass2) / direction.powi(2));
    r.normalize() * f_mag
}
```
Which calculates the force vector on body 1 in reference to body 2,
adapted from Newton's Gravitational Law. The ``gravity_update()`` function
also updates our ``SimulationState`` struct, contained inside
``SimulationService`` struct which gets used with our gRPC server. (No,
that's not the real gravitational constant)

Before moving on to the server, I'd like to highlight a small debugging
system I wrote up so that I could vizualize the direction of the bodies.

```rust 
pub fn setup_vectors(mut commands: Commands, query_bodies: Query<&Transform>) {
    for _ in query_bodies.iter() {
        let line = shapes::Line(Vec2::ZERO, Vec2::new(0.0, 0.0));
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Stroke::new(Color::WHITE, 5.0), // Spawn in lines
        ));
    }
}

pub fn vector_update(query_body: Query<(&Transform, &Velocity)>, mut query_path: Query<&mut Path>) {
    for ((transform, velocity), mut path) in query_body.iter().zip(query_path.iter_mut()) {
        let center_of_mass = transform.translation.truncate();
        let vel = velocity.linvel;
        let new_line = shapes::Line(center_of_mass, center_of_mass + vel);
        *path = ShapePath::build_as(&new_line);
    }
}
```

Now, since the model will not be interested in all of the values that
Rapier needs, we define a seperate ``BodyAttributes`` that our gRPC
service uses. Here is the ``simulation.proto``

```c++
syntax = "proto3";

package simulation;

service Sim {
  rpc Replies(SimReq) returns (SimResponse) {}
}

message SimReq{
  optional string bodyID = 1;
}

message Vec2D{
  float x = 1;
  float y = 2;
}
message BodyAttributes{
  Vec2D velocity = 1;
  Vec2D position = 2;
}
message SimResponse{
  repeated BodyAttributes bodies = 1;
}
```

We create the server as a startup system in the Bevy ECS, and add the
tokio runtime as a resource to the Bevy up so that our server can handle
requests as they come in asynchronously. 

We can connect to it on ``localhost:50051`` and get the velocity and
position of our bodies.
