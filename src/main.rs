use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Enter the width of the map:");
    let width: usize = read_input().unwrap_or_else(|_| {
        println!("Invalid input, please enter a valid integer.");
        std::process::exit(1);
    });
    println!("Enter the height of the map:");
    let height: usize = read_input().unwrap_or_else(|_| {
        println!("Invalid input, please enter a valid integer.");
        std::process::exit(1);
    });
    println!("Enter the number of walkers per cell:");
    let walkers_per_cell: usize = read_input().unwrap_or_else(|_| {
        println!("Invalid input, please enter a valid integer.");
        std::process::exit(1);
    });
    let pixel_size = 20;
    // println!("Enter the size of each pixel:");
    // let pixel_size: usize = read_input().unwrap_or_else(|_| {
    //     println!("Invalid input, please enter a valid integer.");
    //     std::process::exit(1);
    // });
    let iterations = walkers_per_cell * width * height;
    let mut map = vec![vec![false; height]; width];
    let mut x = width / 2;
    let mut y = height / 2;
    let sparse = 1;
    for _ in 0..iterations {
        map[x][y] = true;
        let dx = rng.gen_range(-sparse..=sparse);
        let dy = rng.gen_range(-sparse..=sparse);
        x = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;
        y = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
    }

    let mut buffer: Vec<u32> = vec![0; width * height * pixel_size * pixel_size];
    for x in 0..width {
        for y in 0..height {
            if map[x][y] {
                let color = 0xFFFFFF;
                for i in 0..pixel_size {
                    for j in 0..pixel_size {
                        buffer[(y * pixel_size + j) * width * pixel_size + (x * pixel_size + i)] =
                            color;
                    }
                }
            }
        }
    }

    let mut window = Window::new(
        "Map",
        width * pixel_size/2,
        height * pixel_size/2,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width * pixel_size, height * pixel_size)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}

fn read_input<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse()
}
