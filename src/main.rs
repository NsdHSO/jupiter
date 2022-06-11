
fn main() {
    let e = Box::new(4);
    println!("{}",e);
    println!("{}", calc_weight_on_mars(100.0));
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.71
}
