pub fn image_compare(target: Vec<u8>, image: Vec<u8>) -> f64{
    let mut mse: f64 = image.iter().zip(target).map(|(a, b)| ((*a as f64 - b as f64).powf(2.0))).sum();
    //let mut mse: f64 = (mse as f64).powf(2.0);
    //println!{"{}, {}", target.len(), image.len()}
    return mse / image.len() as f64;
}

pub fn get_mean_point(shape_type: i32, shape_data: Vec<(i32, i32)>) -> (i32, i32) {
    match shape_type {
        0 => return shape_data[1],
        1 | 2 => {
            let x_mean: i32 = shape_data.clone()
                .iter()
                .map(|t| t.0)
                .sum::<i32>()
                / shape_data.len() as i32;
            
            let y_mean: i32 = shape_data.clone()
                .iter()
                .map(|t| t.1)
                .sum::<i32>()
                / shape_data.len() as i32;
            return (x_mean, y_mean);
        },
        3 => return shape_data[0],
        _ => panic!("Gen range returned an unexpected value!?"),
    }
}