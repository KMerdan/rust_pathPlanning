use euclid::{Point2D, Vector2D};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::f32::consts::PI;

use crate::grid_cell::Cell;
use crate::selected_point::Node;

pub fn a_star(
    start: Cell,
    goal: Cell,
    width: usize,
    height: usize,
    buffer: &Vec<Vec<u32>>,
    pixel_size: usize,
) -> Option<Vec<Cell>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let min_distance = 10.0;

    g_score.insert(start, 0.0);
    f_score.insert(start, start.heuristic(&goal, pixel_size));

    open_set.push(Reverse(Node::new(
        start,
        0.0,
        start.heuristic(&goal, pixel_size),
    )));

    while let Some(Reverse(current)) = open_set.pop() {
        if current.cell == goal {
            let mut path = Vec::new();
            let mut current = current.cell;
            while let Some(&prev) = came_from.get(&current) {
                path.push(current);
                current = prev;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for neighbor in current
            .cell
            .neighbors(width, height, buffer, pixel_size, min_distance)
        {
            let tentative_g_score =
                g_score.get(&current.cell).unwrap() + current.cell.heuristic(&neighbor, pixel_size);
            if let Some(&g) = g_score.get(&neighbor) {
                if tentative_g_score >= g {
                    continue;
                }
            }

            came_from.insert(neighbor, current.cell);
            g_score.insert(neighbor, tentative_g_score);
            f_score.insert(
                neighbor,
                tentative_g_score + neighbor.heuristic(&goal, pixel_size),
            );
            open_set.push(Reverse(Node::new(
                neighbor,
                tentative_g_score,
                neighbor.heuristic(&goal, pixel_size),
            )));
        }
    }

    None
}

pub fn bfs(
    start: Cell,
    goal: Cell,
    width: usize,
    height: usize,
    buffer: &mut Vec<Vec<u32>>,
    pixel_size: usize,
) -> Option<Vec<Cell>> {
    let mut queue = VecDeque::new();
    let mut came_from = HashMap::new();
    let mut visited = HashSet::new();
    let min_distance = 10.0;

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = vec![current];
            while let Some(parent) = came_from.get(path.last().unwrap()) {
                path.push(*parent);
            }
            path.reverse();
            return Some(path);
        }

        let neighbors = current.neighbors(width, height, &buffer, pixel_size, min_distance);
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

pub fn bfs_bezier(
    start: Cell,
    goal: Cell,
    width: usize,
    height: usize,
    buffer: &mut Vec<Vec<u32>>,
    pixel_size: usize,
) -> Option<(Vec<Cell>, Vec<Cell>)> {
    let mut queue = VecDeque::new();
    let mut came_from = HashMap::new();
    let mut visited = HashSet::new();
    let min_distance = 10.0;

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = vec![current];
            while let Some(parent) = came_from.get(path.last().unwrap()) {
                path.push(*parent);
            }
            path.reverse();
            let original_path = path.clone();

            // Smooth the path using a Bezier curve
            let smoothed_path = smooth_path(&path, pixel_size);

            return Some((original_path, smoothed_path));
        }

        let neighbors = current.neighbors_old(width, height, &buffer, pixel_size);
        // let neighbors = current.neighbors(width, height, &buffer, pixel_size, min_distance);
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn smooth_path(path: &Vec<Cell>, pixel_size: usize) -> Vec<Cell> {
    // Convert the path to a vector of control points
    let control_points = path
        .iter()
        .map(|&cell| cell.to_point(pixel_size))
        .collect::<Vec<_>>();

    // Iterate over the control points and add additional points to smooth sharp corners
    let mut smoothed_points = vec![];
    for i in 1..control_points.len() - 1 {
        let prev = control_points[i - 1];
        let curr = control_points[i];
        let next = control_points[i + 1];
        let angle =
            (next.y - curr.y).atan2(next.x - curr.x) - (curr.y - prev.y).atan2(curr.x - prev.x);
        if angle.abs() > (PI / 1.0) as f32 {
            let new_point = Point2D::new((curr.x + next.x) / 2.0, (curr.y + next.y) / 2.0);
            smoothed_points.push(prev);
            smoothed_points.push(new_point);
        } else {
            smoothed_points.push(curr);
        }
    }
    smoothed_points.push(*control_points.last().unwrap());

    // Create the Bezier path
    let mut path = vec![];
    for i in 0..smoothed_points.len() - 2 {
        let p0 = smoothed_points[i];
        let p1 = smoothed_points[i + 1];
        let p2 = smoothed_points[i + 2];
        let v1 = Vector2D::new(p1.x - p0.x, p1.y - p0.y);
        let v2 = Vector2D::new(p2.x - p1.x, p2.y - p1.y);
        let angle = v1.angle_to(v2);
        if angle.radians > (PI / 2.0) as f32 {
            let c1 = p1 + v1.normalize() * v1.length() / 3.0;
            let c2 = p1 + v2.normalize() * v2.length() / 3.0;
            path.push(p1);
            path.push(c1);
            path.push(c2);
        } else {
            path.push(p1);
        }
    }
    path.push(*smoothed_points.last().unwrap());

    // Sample the path at a high resolution
    let num_samples = 1000;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / (num_samples - 1) as f32;
        let point = point_on_bezier_path(&path, t);
        samples.push(point);
    }

    // Convert the sampled points back to cells and return them as the smoothed path
    samples
        .iter()
        .map(|&point| Cell::from_point(point, pixel_size))
        .collect::<Vec<_>>()
}

fn point_on_bezier_path(path: &Vec<Point2D<f32, ()>>, t: f32) -> Point2D<f32, ()> {
    let n = path.len() - 1;
    let mut b = path.clone();
    for r in 1..=n {
        for i in 0..=n - r {
            // b[i] = (1.0 - t) * b[i].to_array() + t * b[i + 1].to_array();
            let x = (1.0 - t) * b[i].to_array()[0] + t * b[i + 1].to_array()[0];
            let y = (1.0 - t) * b[i].to_array()[1] + t * b[i + 1].to_array()[1];

            b[i] = Point2D::new(x, y);
        }
    }
    b[0]
}

pub fn floyd_warshall(
    start: Cell,
    goal: Cell,
    width: usize,
    height: usize,
    buffer: &mut Vec<Vec<u32>>,
    pixel_size: usize,
) -> Option<(Vec<Cell>, Vec<Cell>)> {
    // Initialize the distance matrix with infinity for all pairs of cells
    let mut dist = vec![vec![f32::INFINITY; width]; height];
    
    // Set the distance of each cell to itself to 0
    for i in 0..width {
        for j in 0..height {
            if Cell::from_point(Point2D::new(i as f32, j as f32), pixel_size).is_obstacle(&buffer) {
                continue;
            }
            dist[i-1][j-1] = 0.0;
        }
    }
    println!("Dist dimensions: {} x {}", dist.len(), dist[0].len());

    // Update the distance matrix with the actual distances between cells
    for i in 0..width {
        for j in 0..height {
            if Cell::from_point(Point2D::new(i as f32, j as f32), pixel_size).is_obstacle(&buffer) {
                continue;
            }
            let current = Cell::from_point(Point2D::new(i as f32, j as f32), pixel_size);
            let neighbors = current.neighbors_old(width, height, &buffer, pixel_size);
            for neighbor in neighbors {
                let weight = current.distance(&neighbor) as f32;
                dist[current.to_point(pixel_size).x as usize]
                    [current.to_point(pixel_size).y as usize] = weight;
            }
        }
    }

    // Run the Floyd-Warshall algorithm to find the shortest path between all pairs of cells
    for k in 0..width {
        for i in 0..width {
            for j in 0..height {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    // Reconstruct the path from start to goal using the distance matrix
    let mut path = vec![start];
    let mut current = start;
    while current != goal {
        let neighbors = current.neighbors_old(width, height, &buffer, pixel_size);
        let mut next = None;
        let mut min_dist = f32::INFINITY;
        for neighbor in neighbors {
            let dist = dist[neighbor.to_point(pixel_size).x as usize]
                [neighbor.to_point(pixel_size).y as usize];
            if dist < min_dist {
                min_dist = dist;
                next = Some(neighbor);
            }
        }
        if let Some(next_cell) = next {
            path.push(next_cell);
            current = next_cell;
        } else {
            return None;
        }
    }

    // Smooth the path using a Bezier curve
    let smoothed_path = smooth_path(&path, pixel_size);

    Some((path, smoothed_path))
}
