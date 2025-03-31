The three body problem is a relatively well known scenario in classical
mechanics, as well as a pretty good series of novels, with no closed form
solution. It's comprised of three, equal mass, bodies in space (the "three
body" part) and the aim is describing the movement caused by gravitational
attraction between the bodies(the "problem" part). Some restricted variations of the 
problem have solutions, and it's most generally described as a system of
three second order vector differential equations, or a really ugly
eighteen part Hamiltonian differential scalar equations. In an attempt to
avoid the physics, I set out to model the problem in
a close-enough-to-have-fun simulation.

The simulation is set up in Rust, using Bevy + Rapier 2D. I hesitated to commit
to using Bevy since I thought it would be overkill for what I needed it for,
and that was somewhat true, but this [particularly interesting](https://www.youtube.com/watch?v=PND2Wpy6U-E) podcast convinced me
to bite the bullet and figure out Bevy's ECS system. With a little bit of
hindsight, I can say confidently it would have been easier to use Rapier
as a headless physics solver and simply visualize using some plotting
library down the pipeline. If I wanted to eventually *solve* the problem,
I needed a way for the program to communicate with external program;
since, unfortunatly, [we are not learning
yet](https://www.arewelearningyet.com/) and I also just wanted to play
around with IPC. I chose to use gRPC by way of Tokio's Tonic since
I already had the mind to write somewhat effecient code considering the
computational demand of the problem. To complete the chain, I wrote up
a small python client to interact with the simulation.

The simulation is done in the system ```gravity_update()```, which, for
every combination of two bodies, fetches the mass and location of the
bodies, then calculates and applies the gravitational force and its
opposite that the bodies apply on each other. The ```gravitational_force()```,
calculation is adapted from Newton's Gravitational Law.
(No, that's not the real gravitational constant)


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

Briefly, I added vectors to show more obviously the velocities of the
bodies during the simulation.

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

The gRPC server handles the setting of initial conditions (the solution,
as it were), as well as resetting the simulation and handing out the
current state of the simulation. The ```SimState``` message is simply an
array of ```BodyAttributes```, values necessary to make the calculations
for the simulation.

```c++
syntax = "proto3";

package simulation;

service Sim {
  rpc StateReply(SimCurrentStateReq) returns (SimState) {}
  rpc SetConfiguration(SimState) returns (ConfigValid) {}
}

message SimCurrentStateReq{
  optional string bodyID = 1;
}

message Vec2D{
  float x = 1;
  float y = 2;
}

message ConfigValid{
  bool succeeded = 1;
}

message BodyAttributes{
  Vec2D velocity = 1;
  Vec2D position = 2;
  float mass = 3;
  uint32 bodyID = 4;
}

message SimState{
  repeated BodyAttributes bodies = 1;
}
```

The Tonic Server is attached as a startup system in the Bevy ECS, and the
tokio runtime is added as a resource so the server can handle requests as
they come in asynchronously. We can connect to it on ``localhost:50051``
and get the velocity and position of our bodies. 

The client on the reciving end of the gRPC contract is implemented in
a seperate python script.

```python
def set_configuration(bodies, stub):
    """
    Calls the SetConfiguration RPC to set the simulation configuration.
    """
    sim_state = simulation_pb2.SimState(
        bodies=[
            simulation_pb2.BodyAttributes(
                bodyID=body["bodyID"],
                position=simulation_pb2.Vec2D(
                    x=body["position"]["x"], y=body["position"]["y"]
                ),
                velocity=simulation_pb2.Vec2D(
                    x=body["velocity"]["x"], y=body["velocity"]["y"]
                ),
                mass=body["mass"],
            )
            for body in bodies
        ]
    )
    response = stub.SetConfiguration(sim_state)


def body_request(stub):
    """
    Requests the location, velocity, and mass of bodies in the current simulation
    """
    request = simulation_pb2.SimCurrentStateReq()
    try:
        response = stub.StateReply(request)
        return response
    except grpc.RpcError as e:
        print(f"gRPC call failed: {e.code()}: {e.details()}")
```

Using the dictionary that the service provides, the client evaluates if
the simulation has effectively ended, or if the initial configuration no
longer has the chance of being stable. The "fail states" of the simulation
occur when either there was a collision or if one of the bodies escapes
from the orbit of the other two.

```Python
def collision(bodies):
    """
    returns true if body positions are within radii of one another
    """
    combinations = itertools.combinations(bodies, 2)
    collision_flag = false
    for combination in combinations:
        b1, b2 = combination
        vec1 = np.array([b1.position.x, b1.position.y])
        vec2 = np.array([b2.position.x, b2.position.y])
        distance = np.linalg.norm(vec1 - vec2)
        if round(distance, 2) <= 80.0:
            collision_flag = true

    return collision_flag


def escape(bodies):
    """
    returns true if a body is no longer effect by the gravitational force of one of the others
    """
    combinations = itertools.combinations(bodies, 2)
    distance_flag = false
    for combination in combinations:
        b1, b2 = combination
        vec1 = np.array([b1.position.x, b1.position.y])
        vec2 = np.array([b2.position.x, b2.position.y])
        distance = np.linalg.norm(vec1 - vec2)
        g_f = (b1.mass * b2.mass) / (distance**2)
        if round(g_f, 3) == 0:
            distance_flag = true
    return distance_flag
```

I already implemented a simple genetic algorithm hoping to find some
stable configurations, it iterated with mild success, confirming that the
simulation-solver set up was working, although the method itself failed
due to its own simplicity, as each generation simply edged further away
while not triggering the ```escape()``` fail condition. From here I plan
to integrate a gradient descent algorithm with the formulization described
[here](https://www.sciencedirect.com/science/article/pii/S1384107622001737).
If you are reading this, I have not done so yet, au revouir. 
