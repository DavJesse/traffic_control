# Road Intersection

## Overview

This project simulates a traffic control system at a four-way road intersection. It features traffic lights, vehicle spawning, and collision-free navigation. The simulation is rendered using SDL2, and vehicles follow fixed routes through the intersection with the goal of minimizing congestion and avoiding collisions.

## Getting Started

Clone this repository and navigate into the folder::

```bash
git clone https://learn.zone01kisumu.ke/git/bshisia/road_intersection.git
cd road_intersection
```

To run the program, use the following command:

```bash
cargo run
```

## Simulation Controls

Use the following keyboard keys to interact with the simulation:

    Key        |          Action
    ↑ Up       | Spawn vehicle from South (heading North)
    ↓ Down     | Spawn vehicle from North (heading South)
    → Right    | Spawn vehicle from West (heading East)
    ← Left     | Spawn vehicle from East (heading West)
    r          | Spawn vehicle from a random direction
    Esc        | Exit simulation
Vehicles are spawned with safe spacing to prevent immediate collisions.


## Example Output

![Road Intersection](interestion.gif)

## Authors

- [Brian Shisia](https://github.com/BrianShisia)
- [Joab Owala](https://github.com/jowala)
- [David Jesse Odhiambo](https://github.com/DavJesse)
