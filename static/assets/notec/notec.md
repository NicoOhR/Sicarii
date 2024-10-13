To start, FSAE (Formula: Student Automotive Engineer) is a worldwide
competition aimed primarily at giving university students a competition to
exercise and apply the skills which they learn in the classroom. The
competition is aimed predominantly at Mechanical Engineering, and in
recent years with the advent of the EV competition, Electrical Engineering
majors. By way of demand, however, the FSAE chapter at UT Dallas, a school
overfilled with Computer Science Majors, several large software and
firmware projects. Here I focus on the project which I have spent the most
time on, the Data Acquisition Device (colloquially, NoTeC, a joke I will
explain later) and will hopefully circle back to the other projects later.

The purpose of NoTeC is to sample different sensors through out the car,
and in some way get it to the engineers. How we serve the data to the
engineers has become increasingly more intricate and seamless over time.
Originally, the DAQ saved to a USB drive in CSV form, then it saved it to
an adjacent Raspberry Pi device, then the Raspberry Pi began sending and
syncing the data to a MongoDB database that can be quaried. Live telemetry
using Grafana is in the works, although theres some practicality issues
with getting WiFi to the Raspberry Pi without breaking the rules of the
competition.

The DAQ itself is currently implemented on an STM32F4 chip, but I am
currently migrating it to the STM32H7, on the basis of better performance.
It's written with the heavy help ST's HAL and C++. The impetus towards
using C++ is predominantly the ease of smart pointers, which allow me to
more easily make the system modular and hardware independent (we'll get
back to this). CMSIS-RTOS v2 is used for the threading.

The project structure:

```
.
├── Application
│   ├── DataLogger
│   ├── FileSystem
│   ├── Mutex
│   └── Relay
├── Docs
│   ├── data_logging
│   ├── engine_control_unit
│   ├── file_system
│   ├── linear_potentiometer
│   ├── peripheral_interfaces
│   ├── _static
│   └── stmicroelectronics
├── Platform
│   ├── Interfaces
│   └── STM
│       └── F4
│           ├── CAN
│           ├── GPIO
│           └── I2C
├── Sensor
│   ├── Accelerometer
│   ├── ECU
│   │   └── PE3
│   │       └── Frames
│   ├── GyroScope
│   └── LinearPotentiometer
└── Tests
    └── Src
        ├── Application
        └── Sensor
            └── ECU
                └── PE3
```

The project is split into five parts which are, in order of their
proximity to "bare metal": Platform, Sensor, Application, and as auxiliary
systems, Tests and Docs. In the platform layer, we abstract away some of
the rougher edges of the HAL, making writing the actual application of the
DAQ a little less error prone. This also benefits the application insofar
as it decouples from the hardware. If I wanted to switch to an NXP chip
instead, I would only have to re implement this layer, as the Application
and Sensor layer uses the interfaces we make here.

The next layer, Sensor, does exactly what it sounds like, this is where we
create the sensor classes who interact with the interfaces we created in
the Platform layer. There also exists abstract interfaces for sensors,
hopefully making it easier to switch out the specific sensor used when we
need to, however we haven't really had the opportunity to put this to the
test as we haven't upgraded sensors in a long while.

Lastly is the Application layer, which handles the state of the device,
our mutexes for multi-threading, and how we talk to the Raspberry Pi (the
Relay class). 

The Tests folder has some preliminary unit tests made with gtest that are
hooked up to our GitHub CI/CD pipeline, but as with other "it'd be nice to
have" things in this project, I've yet to give it the time that it
deserves. Similarly the Docs folder has some documentation generation with
Phoenix, but I've yet to find the patience to annotate the existing code.
