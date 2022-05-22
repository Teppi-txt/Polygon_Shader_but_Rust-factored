/* shape map:
0 - circle
1 - rect
2 - polygon
3 - cross
4 - line
 */

pub fn generate_shape_data(shape_type: i32, width: i32, height: i32) -> Vec<(i32, i32)>{
    use rand::{thread_rng, Rng};
    //circle data stored as follows: "circle", "circle width and height", "center"
    let mut rng = thread_rng();

    match shape_type {
        //[(center_x, center_y), (radius_x, radius_y)]
        0 => return vec![(rng.gen_range(0..=width), rng.gen_range(0..=height)), (rng.gen_range(1..=width/2), rng.gen_range(1..=height/2))],

        //[(start_x, start_y), (width, width_y)]
        1 => return vec![(rng.gen_range(0..=width), rng.gen_range(0..=height)), (rng.gen_range(1..=width), rng.gen_range(1..=height))],

        2 => {
            let mut poly_list = Vec::new();
            for point in 1..=rng.gen_range(3..=4) {
                poly_list.push((rng.gen_range(0..=width), rng.gen_range(0..=height)));
            }
            return poly_list;
        },
        3 => return vec![(rng.gen_range(0..=width), rng.gen_range(0..=height))],
        _ => panic!("Gen range returned an unexpected value!?"),
    }
}

pub fn generate_shape_type() -> i32 {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    return rng.gen_range(0..=2);
}

pub fn generate_shape_color() -> [u8; 4] {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    return [rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)];
}

pub fn shift_shape_data(old_data: Vec<((i32, i32))>) -> Vec<((i32, i32))> {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut new_data: Vec<(i32, i32)> = Vec::new();
    let evo_factor = 40;

    for point in old_data.clone() {
        new_data.push((std::cmp::max(0, point.0 + rng.gen_range(0..=evo_factor)), std::cmp::max(0, point.0 + rng.gen_range(0..=evo_factor))));
    }
    return new_data;
}

pub fn shift_shape_color(old_color: [u8; 4]) -> [u8; 4] {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let evo_factor = 20;
    return [old_color[0] + rng.gen_range(0..=evo_factor),  old_color[1] + rng.gen_range(0..=evo_factor), old_color[2] + rng.gen_range(0..=evo_factor), old_color[3] + rng.gen_range(0..=evo_factor)];
}