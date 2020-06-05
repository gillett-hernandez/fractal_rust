//! An example of generating julia fractals.
extern crate image;
extern crate minifb;
extern crate num_complex;

use minifb::{Key, Window, WindowOptions};

use rand::prelude::*;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use std::time::Duration;

fn mandelbrot_set() {}

fn main() {
    let imgx = (2048.0 * 1.0) as u32;
    let imgy = 2048;

    let aspect_ratio: f64 = (imgx as f64) / (imgy as f64);

    let aa_steps = 1;
    // let mut topleft = (-2.0, -1.0);
    // let mut size: f64 = 2.0;
    let mut topleft = (-0.1011, 0.9563);
    let mut size: f64 = 0.01;

    let (pixel_width, pixel_height) = ((aspect_ratio * size) / imgx as f64, size / imgy as f64);

    ///////////////////////////////

    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut pixel_i = 0;
        for (pixel) in buffer.iter_mut() {
            let mut i = 0;
            let (x, y) = (pixel_i % WIDTH, pixel_i / WIDTH);
            for _ in 0..aa_steps {
                let cx = ((x as f64 + random::<f64>()) * pixel_width + topleft.0);
                let cy = ((y as f64 + random::<f64>()) * pixel_height + topleft.1);

                let c = num_complex::Complex::new(cx, cy);
                let mut z = num_complex::Complex::new(0.0, 0.0);

                let mut local_i = 0;
                while local_i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    local_i += 1;
                    i += 1;
                }
            }

            pixel_i += 1;

            // let image::Rgb(data) = *pixel;
            // *pixel = image::Rgb([data[0], (i as f64 / aa_steps as f64) as u8, data[2]]);
            *pixel = ((i as f64 / aa_steps as f64) as u32) << 8;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
