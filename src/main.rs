use std::io;

fn main() {
    println!("Please introduce some char: ");
    let mut key = String::new();
    io::stdin().read_line(&mut key);
    pointer_owner(&key);
    let e = Box::new(4);
    println!("{}", e);
    println!("{}", 1000.0 * calc_weight_on_mars(10.9));
    
    println!("{}", gcd(3231, 32312));
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.71
}

fn pointer_owner(s: &String) {
    println!("TEST : {}", s);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m<n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
