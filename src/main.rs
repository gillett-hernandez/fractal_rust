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
    const WIDTH: usize = 1800;
    const HEIGHT: usize = 900;
    // let imgx = (1920.0 * 1.0) as u32;
    let imgx = WIDTH as u32;
    // let imgy = 1080 as u32;
    let imgy = HEIGHT as u32;

    let aspect_ratio: f64 = (imgx as f64) / (imgy as f64);

    let aa_steps = 0;
    let mut topleft = (-2.0, -1.0);
    let mut size: f64 = 2.0;
    // let mut topleft = (-0.1011, 0.9563);
    // let mut size: f64 = 0.01;

    let (pixel_width, pixel_height) = ((aspect_ratio * size) / imgx as f64, size / imgy as f64);

    ///////////////////////////////

    // colors as RGBA unsigned 32 bit integers
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut complex_buffer = vec![num_complex::Complex::new(0.0, 0.0); WIDTH * HEIGHT];
    let mut iter_buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~144 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(6944)));

    let mut aa_step = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for (pixel) in buffer.iter_mut() {
        if aa_step < aa_steps || aa_steps == 0 {
            for pixel_i in 0..(HEIGHT * WIDTH) {
                let mut pixel = &mut buffer[pixel_i];
                let mut i = 0;
                let (x, y) = (pixel_i % WIDTH, pixel_i / WIDTH);
                let cx = ((x as f64 + random::<f64>()) * pixel_width + topleft.0);
                let cy = ((y as f64 + random::<f64>()) * pixel_height + topleft.1);

                let c = num_complex::Complex::new(cx, cy);

                let mut local_i = 0;
                while local_i < 255 && complex_buffer[pixel_i].norm() <= 2.0 {
                    complex_buffer[pixel_i] = complex_buffer[pixel_i] * complex_buffer[pixel_i] + c;
                    local_i += 1;
                }

                iter_buffer[pixel_i] += local_i;

                // let image::Rgb(data) = *pixel;
                // *pixel = image::Rgb([data[0], (i as f64 / aa_steps as f64) as u8, data[2]]);
                *pixel = ((iter_buffer[pixel_i] as f64 / (1 + aa_step) as f64) as u32) << 16;
                // }
            }
            aa_step += 1;
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        } else {
            window.update();
        }
    }
}
