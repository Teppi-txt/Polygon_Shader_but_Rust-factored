pub fn image_compare(target: Vec<u8>, image: Vec<u8>) -> f64{
    let mut mse: f64 = image.iter().zip(target).map(|(a, b)| ((*a as f64 - b as f64).powf(2.0))).sum();
    //let mut mse: f64 = (mse as f64).powf(2.0);
    //println!{"{}, {}", target.len(), image.len()}
    return (mse / image.len() as f64);
}

