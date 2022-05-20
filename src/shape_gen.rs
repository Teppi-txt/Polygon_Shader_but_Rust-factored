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
        0 => return vec![(rng.gen_range(0..=width), rng.gen_range(0..=height)), (rng.gen_range(0..=width/2), rng.gen_range(0..=height/2))],

        //[(start_x, start_y), (width, width_y)]
        1 => return vec![(rng.gen_range(0..=width), rng.gen_range(0..=height)), (rng.gen_range(0..=width), rng.gen_range(0..=height))],

        2 => {
            let mut poly_list = Vec::new();
            for point in 1..=rng.gen_range(3..=4) {
                poly_list.push((rng.gen_range(0..=width), rng.gen_range(0..=height)));
            }
            return poly_list;
        }
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