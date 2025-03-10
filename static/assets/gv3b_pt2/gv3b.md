And we're back. I ended up making some changes to the simulation rig since
the last time I posted about this project. I ended up ditching the TOML
configuration, opting instead for the client to send a configuration, like
so:

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

I ended up changing some names too, as well as adding some more detail to
the ``BodyAttributes`` message. Since the RPC contract between the server
and client needs to be sync'd, and I also wanted these two programs to be
as decoupled as possible, I ended up using PyBuilder to fetch the latest
from the repository:

```Python
@task
def get_protobuf_service_def(project):
    protobuf_url = "https://raw.githubusercontent.com/NicoOhR/GV3B_simulation/refs/heads/main/proto/simulation.proto"
    response = requests.get(protobuf_url)
    if response.status_code == 200:
        proto_dir = os.path.join(
            project.get_property("dir_source_main_python"), "proto"
        )
        proto_file = os.path.join(proto_dir, "simulation.proto")
        project.set_property("proto_dir", proto_dir)
        project.set_property("proto_file", proto_file)
        os.makedirs(proto_dir, exist_ok=True)
        with open(proto_file, "wb") as file:
            file.write(response.content)
            print("[Builder] Successfuly got latest protobuf")
    else:
        print("[Builder] Failed to get protobuf")
        print(response.status_code)


@task
def build_proto_buf(project):
    proto_file = project.get_property("proto_file")
    proto_dir = project.get_property("proto_dir")
    module_file = os.path.join(proto_dir, "__init__.py")
    command = [
        "python3",
        "-m",
        "grpc_tools.protoc",
        "-I",
        proto_dir,
        f"--python_out={proto_dir}",
        f"--grpc_python_out={proto_dir}",
        proto_file,
    ]
    ...
```

I fully recognize and appreciate that having a build system in a python
project might just be me being massively C-brained, but it gets the job
done and organizes the project pretty well. The actual client, being only
two methods, is pretty simple:

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

Using the dictionary that the service gives us, we evaluate if the
simulation has effectively ended, or when the configuration no longer has
the chance of being stable. For our purposes that is when a collision has
occured, or when one of the bodies has escaped the gravitational force of
the others. That looks something like this:

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

In the same file I define some banal functions, ``runtime`` and
``create_config``, both of which do exactly what they sound like. We use
our runtime as the fitness matrix which we evolve our body configurations
with. Using the DEAP framework, I whipped up this genetic algorithm that
works... servicably

```Python

fitnesses = list(map(toolbox.evaluate, population))
for ind, fit in zip(population, fitnesses):
ind.fitness.values = fit

for gen in range(num_generations):
offspring = toolbox.select(population, len(population))
offspring = list(map(toolbox.clone, offspring))

for child1, child2 in zip(offspring[::2], offspring[1::2]):
    if np.random.rand() < crossover_prob:
        toolbox.mate(child1, child2)
        del child1.fitness.values
        del child2.fitness.values

for mutant in offspring:
    if np.random.rand() < mutation_prob:
        toolbox.mutate(mutant)
        del mutant.fitness.values

invalid_ind = [ind for ind in offspring if not ind.fitness.valid]
fitnesses = map(toolbox.evaluate, invalid_ind)
for ind, fit in zip(invalid_ind, fitnesses):
    ind.fitness.values = fit

population[:] = offspring

fits = [ind.fitness.values[0] for ind in population]
```

the algorithm is pretty textbook (DEAP textbook to be specific). But it
does iterate, slowly. I tried using the SciPy optimize package, but that
never got up and running, and back propogation never made it back up the
computation graph when using PyTorch (since, surprisingly, gRPC is not
auto-differentiable). The problem that I ran into with this algorithm,
only couple of hours of iteration into the whole thing, is that the
individuals which maximized the fitness function were the ones on the very
edges of one anothers gravitational field. Their offspring, in turn, would
be just exactly outside of the field, reseting the simulation.

Some rethinking needs to be done on this, and I might just ditch the
external simulation, and instead write the physics solver myself in an
entierly differentiable manner. Still, I'm not sure what the appropriate
cost function would be, I flirted briefly with a Lyapunov based cost
function, but by the very nature of the problem, there cannot be an
equilibirium point. My running theory is I could probably modify the above
genetic algorithm to train the neural network to find somewhat stable
configurations.

Until I figure out the finer points of dynamical systems, I choose to put
this one on the back bench.
