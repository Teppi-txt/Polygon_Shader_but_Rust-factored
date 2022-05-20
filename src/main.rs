use std::io::Cursor;
use std::env;
use image::{imageops::FilterType, ImageFormat, io::Reader, open};
use imageproc::drawing::{draw_cross_mut, draw_filled_ellipse_mut, draw_filled_circle_mut};
use std::path::PathBuf;
mod shape_gen;


fn main() {
    let target_image = open("src/input/img.jpg").unwrap();

    let img_x: i32 = 758;
    let img_y: i32 = 500;

    let mut imgbuf = image::ImageBuffer::new(img_x as u32, img_y as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255 as u8, 225 as u8, 255 as u8]);
    }


    for i in 0..100{
        let shape_data = shape_gen::generate_shape_data(0, img_x, img_y);
        let shape_color = shape_gen::generate_shape_color();
        println!("{:#?}", shape_data);
        draw_filled_circle_mut(&mut imgbuf, shape_data[0], shape_data[1].0, image::Rgb(shape_color));
        println!{"done"};
    }
    
    imgbuf.save("src/output/woah.png").unwrap();
}