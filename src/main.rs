use std::io::Cursor;
use std::env;
use image::{imageops::FilterType, ImageFormat, io::Reader, open};
use imageproc::rect::Rect;
use imageproc::point::Point;
use imageproc::drawing::{draw_cross_mut, draw_filled_ellipse_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_polygon_mut};
use rand::{thread_rng, Rng};
use std::path::PathBuf;
mod shape_gen;
mod math;

#[derive(Debug, Clone)]
struct Shape {
    id: i32,
    data: Vec<(i32, i32)>,
    color: [u8; 4],
    fitness: f64
}

fn main() {
    let target_image = open("src/input/img.png").unwrap();

    let img_x: i32 = 728;
    let img_y: i32 = 500;

    let mut imgbuf = image::ImageBuffer::new(img_x as u32, img_y as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255 as u8, 225 as u8, 255 as u8]);
    }
    //println!{"{:#?}", imgbuf.clone().into_vec()};
    imgbuf.save("src/output/output.png").unwrap();

    /* for i in 0..100{
        get_best_shape(5, 10, 10);
    } */
    get_best_shape(700, 10, 10);
}

fn get_best_shape(shapes_per_gen: i32, child_n: usize, shapes_survive: i32) {
    let mut image = open("src/output/output.png").unwrap();
    let width = image.width() as i32; let height = image.height() as i32;
    let mut shape_list: Vec<Shape> = Vec::new();
    //println!{"{:#?}", target_image.as_bytes().to_vec()};

    //generates the first generation shapes
    for shape in 0..=shapes_per_gen {
        let shape_type = shape_gen::generate_shape_type();
        let shape_data = shape_gen::generate_shape_data(shape_type, width, height);
        let shape_color = shape_gen::generate_shape_color();

        shape_list.push(draw_shape(shape_type, shape_data, shape_color));
    }

    for gen in 0..child_n {
        //sorts the shape list
        shape_list.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        shape_list.truncate(shapes_survive as usize);
        shape_list = mutate(&shape_list, child_n);
    }
}

fn draw_shape(shape_type: i32, shape_data: Vec<(i32, i32)>, shape_color: [u8; 4]) -> Shape {
    let mut target_image = open("src/input/img.png").unwrap();
    let mut comparison_image = open("src/output/output.png").unwrap().clone();
    match shape_type {
        0 => draw_filled_circle_mut(&mut comparison_image, shape_data[0], shape_data[1].0, image::Rgba(shape_color)),
        1 => draw_filled_rect_mut(&mut comparison_image, Rect::at(shape_data[0].0, shape_data[0].1).of_size(shape_data[1].0 as u32, shape_data[1].1 as u32), image::Rgba(shape_color)),
        2 => {
            let mut poly_list = Vec::new();
            for point in &shape_data {
                poly_list.push(Point::new(point.0, point.1));
            }
            draw_polygon_mut(&mut comparison_image, &poly_list, image::Rgba(shape_color));
        },
        3 => draw_cross_mut(&mut comparison_image, image::Rgba(shape_color), shape_data[0].0 ,shape_data[0].1),
        _ => panic!("shape type returned an unexpected value!?"),
    }
    return Shape{id: shape_type, data: shape_data, color: shape_color, fitness: math::image_compare(target_image.as_bytes().to_vec(), comparison_image.as_bytes().to_vec())}
}

fn mutate(shape_list: &Vec<Shape>, child_n: usize) -> Vec<Shape> {
    let old_list = shape_list.to_vec();

    //add all surviving objects to new list
    let mut new_list = old_list.clone();

    //FOR EVERY SURVIVING SHAPE, MUTATE X TIMES
    for shape in old_list {
        for child in 0..child_n{
            let new_data = shape_gen::shift_shape_data(shape.data.clone());
            let new_color = shape_gen::shift_shape_color(shape.color.clone());
            //println!{"{:#?} -> {:#?} {:#?} {:#?}", shape, shape.id.clone(), new_data, new_color};
            new_list.push(draw_shape(shape.id.clone(), new_data, new_color));
        }
    };
    return new_list;
}