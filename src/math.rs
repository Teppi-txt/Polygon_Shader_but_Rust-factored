pub fn image_compare(target: Vec<u8>, image: Vec<u8>) -> i32{
    let mut mse: i32 = 0;
    //println!{"{}, {}", target.len(), image.len()}
    return image.iter().zip(target).map(|(a, b)| *a as i32 - b as i32).sum();;
}

