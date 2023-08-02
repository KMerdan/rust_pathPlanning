use minifb::{Key, Window, WindowOptions};
use noise::{NoiseFn, Perlin};
use rand::Rng;

mod grid_cell;
mod pathfinder;
mod selected_point;

use grid_cell::Cell;
use pathfinder::{a_star, bfs, bfs_bezier};

fn main() {
    let mut rng = rand::thread_rng();
    let width: usize = 800;
    let height: usize = 600;
    let pixel_size = 10;
    let block_width = width / pixel_size;
    let block_height = height / pixel_size;
    let seed = rng.gen();
    let perlin = Perlin::new(seed);
    let mut buffer: Vec<Vec<u32>> = vec![vec![0; block_width]; block_height];
    let frequency = 4.5;
    let amplitude = 2.5;
    for x in 0..block_width {
        for y in 0..block_height {
            let value = perlin.get([
                (x as f64) / 100.0 * frequency,
                (y as f64) / 100.0 * frequency,
            ]) * amplitude;
            let color = if value > 0.5 { 0xFFFFFF } else { 0x000000 };
            buffer[y][x] = color;
        }
    }

    let mut window =
        Window::new("Map", width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Generate random start and goal positions in free space
    let mut start_x;
    let mut start_y;
    let mut goal_x;
    let mut goal_y;
    loop {
        start_x = rng.gen_range(0..block_width / 2);
        start_y = rng.gen_range(0..block_height / 2);
        goal_x = rng.gen_range(block_width / 2..block_width);
        goal_y = rng.gen_range(block_width / 2..block_height);
        let start_color = buffer[start_y][start_x];
        let goal_color = buffer[goal_y][goal_x];
        if start_color == 0 && goal_color == 0 && (start_x != goal_x || start_y != goal_y) {
            break;
        }
    }

    let mut scaled_buffer: Vec<u32> = vec![0; width * height];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..block_width {
            for y in 0..block_height {
                let color = buffer[y][x];
                for i in 0..pixel_size {
                    for j in 0..pixel_size {
                        scaled_buffer[(y * pixel_size + j) * width + x * pixel_size + i] = color;
                    }
                }
            }
        }

        if let Some((original_path, smoothed_path)) = bfs_bezier(
            Cell {
                block_x: start_x,
                block_y: start_y,
            },
            Cell {
                block_x: goal_x,
                block_y: goal_y,
            },
            width,
            height,
            &mut buffer,
            pixel_size,
        ) {
            let original_path_color = 0x800080; // purple
            let smoothed_path_color = 0xFFA500; // orange
        
            // Plot the original path
            for cell in &original_path {
                for i in 0..pixel_size {
                    for j in 0..pixel_size {
                        scaled_buffer[((cell.block_y * pixel_size + j) * width
                            + cell.block_x * pixel_size
                            + i) as usize] = original_path_color;
                    }
                }
            }
        
            // Plot the smoothed path
            for cell in &smoothed_path {
                for i in 0..pixel_size {
                    for j in 0..pixel_size {
                        scaled_buffer[((cell.block_y * pixel_size + j) * width
                            + cell.block_x * pixel_size
                            + i) as usize] = smoothed_path_color;
                    }
                }
            }
        }

        // Draw start and goal positions on the map
        let start_color = 0xFF0000; // red
        let goal_color = 0x00FF00; // green
        for i in 0..pixel_size {
            for j in 0..pixel_size {
                scaled_buffer
                    [((start_y * pixel_size + j) * width + start_x * pixel_size + i) as usize] =
                    start_color;
                scaled_buffer
                    [((goal_y * pixel_size + j) * width + goal_x * pixel_size + i) as usize] =
                    goal_color;
            }
        }
        window
            .update_with_buffer(&scaled_buffer, width, height)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}
