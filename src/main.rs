//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;

use rand::prelude::*;

fn main() {
    let imgx = 2048;
    let imgy = 2048;

    let scalex = 1.0 / imgx as f32;
    let scaley = 1.0 / imgy as f32;
    let scale: f32 = 3.0;
    let aa_steps = 3;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (128.0 * scalex * x as f32) as u8;
        let b = (128.0 * scaley * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // for ix in 0..10 {
    let ix = 0;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut i = 0;
        for aai in 0..aa_steps {
            let cx = ((x as f32 + random::<f32>()) * scalex - 0.7) * scale;
            let cy = ((y as f32 + random::<f32>()) * scaley - 0.5) * scale;

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
        *pixel = image::Rgb([data[0], (i as f32 / aa_steps as f32) as u8, data[2]]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(format!("fractal{}.png", ix)).unwrap();
    // }
}
