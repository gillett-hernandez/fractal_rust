//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;

use rand::prelude::*;

fn main() {
    let imgx = (2048.0 * 1.5) as u32;
    let imgy = 2048;

    let aspect_ratio: f64 = (imgx as f64) / (imgy as f64);

    let aa_steps = 4;
    let mut topleft = (-2.0, -1.0);
    let mut size: f64 = 2.0;
    let (pixel_width, pixel_height) = ((aspect_ratio * size) / imgx as f64, size / imgy as f64);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (r, b) = (0, 0);
        *pixel = image::Rgb([r, 0, b]);
    }

    // for ix in 0..10 {
    let ix = 0;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut i = 0;
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

        let image::Rgb(data) = *pixel;
        *pixel = image::Rgb([data[0], (i as f64 / aa_steps as f64) as u8, data[2]]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(format!("fractal{}.png", ix)).unwrap();
    // }
}
