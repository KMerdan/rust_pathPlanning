use minifb::{Key, Window, WindowOptions};
use noise::{NoiseFn, Perlin};
use std::io;
use rand::Rng;

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
    let seed = rng.gen();
    let perlin = Perlin::new(seed);
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut frequency = 0.7;
    let mut amplitude = 1.5;
    for x in 0..width {
        for y in 0..height {
            let value = perlin.get([(x as f64) / 100.0 * frequency, (y as f64) / 100.0 * frequency]) * amplitude;
            let color = if value > 0.5 { 0xFFFFFF } else { 0x000000 };
            buffer[y * width + x] = color;
        }
    }

    let pixel_size = 10;
    let mut window = Window::new(
        "Map",
        // width * pixel_size,
        // height * pixel_size,
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut scaled_buffer: Vec<u32> = vec![0; width * height * pixel_size * pixel_size];
        for x in 0..width {
            for y in 0..height {
                let color = buffer[y * width + x];
                for i in 0..pixel_size {
                    for j in 0..pixel_size {
                        scaled_buffer[(y * pixel_size + j) * width * pixel_size + x * pixel_size + i] = color;
                    }
                }
            }
        }
        window
            .update_with_buffer(&scaled_buffer, width * pixel_size, height * pixel_size)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}

fn read_input<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse()
}