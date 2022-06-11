use std::io;

fn main() {
    println!("Please introduce some char: ");
    let mut key = String::new();
    io::stdin().read_line( key);
    pointer_owner(&mut key);
    let e = Box::new(4);
    println!("{}",e);
    println!("{}", 1000.0 *calc_weight_on_mars(10.9));
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.71
}

fn pointer_owner(s: String){
    println!("TEST : {}",s)
}
