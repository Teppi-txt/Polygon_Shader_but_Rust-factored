pub fn generate_shape_data(shape_type: i32, width: i32, height: i32) -> Vec<(i32, i32)>{
    use rand::{thread_rng, Rng};
    //circle data stored as follows: "circle", "circle width and height", "center"
    let mut rng = thread_rng();
    return vec![(rng.gen_range(0..=width), rng.gen_range(0..=height)), (rng.gen_range(0..=width/2), rng.gen_range(0..=height/2))];
}

pub fn generate_shape_type() -> i32 {
    use rand::{thread_rng, Rng};

    let shape_type = 0;//rng.gen_range(0..=0);
    return shape_type;
}