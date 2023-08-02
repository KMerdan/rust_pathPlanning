
## Description
This Rust code demonstrates how to generate a 2D map using Perlin noise and find a path between two points on the map using various pathfinding algorithms.

the `minifb` library is used to visulization.

## Dependencies

This code uses the following Rust libraries:

- `minifb`: A cross-platform windowing and input library.
- `noise`: A procedural noise generation library.
- `rand`: A random number generation library.

## Usage

To run the code, simply compile and execute with `cargo run`. The resulting window will display the generated map and the path between the two randomly generated points.

## Code Structure

The code is divided into several modules:

- `grid_cell`: Defines a `Cell` struct representing a single cell on the map.
- `pathfinder`: Contains various pathfinding algorithms, including A* and BFS.
- `selected_point`: Defines a `SelectedPoint` enum representing the start and goal points on the map.

The `main` function generates the map using Perlin noise, generates random start and goal points, and displays the map and path in a window using `minifb`. The `bfs_bezier` function in the `pathfinder` module is used to find the path between the start and goal points.