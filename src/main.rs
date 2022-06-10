#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 5;
    let coords: (f32, f32) = (6.3, 15.0);
    let name: String = String::from("IOn");
    let name = String::from("HI");
    let s2 = name;

    let mut arrow_coords: Vec<i32> = Vec::new();

    arrow_coords.push(1);

    let mut apples = Grapes { grapesLeft: 33 };

    apples.bite();
    println!("{}", apples.grapesLeft);
    let mut arg: String = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Please Supply an argument");
            std::process::exit(-1);
        })
        .to_owned();

    inspect(&arg);
    change(&mut arg);
    println!("{}", eat(arg));
    println!("{:?}", apples);
    for num in [1, 2, 3, 4].iter() {
        print!("{}", num);
    }

    {
        let area = area(width, height);
        println!("{}", area);
    }

    print!(
        "Diff from x and y is = {} ",
        print_difference(coords.0, coords.1)
    );

    println!("{}", three_multiple(3, 3, 3));
}

pub fn area(width: i32, height: i32) -> i32 {
    width * height
}

pub fn three_multiple(number1: i32, number2: i32, number3: i32) -> i32 {
    number1 * number2 * number3
}

pub fn print_difference(x: f32, y: f32) -> f32 {
    y - x
}

pub fn read_args() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        if arg == "sum" {
            sum()
        } else if arg == "double" {
            double()
        } else {
            println!("Count fct, {}", arg);
        }
    }
}

fn sum() {
    let mut sum = 0;

    for i in 7..=23 {
        sum += i;
    }
    println!("{}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x < 100 {
        count += 1;
        x *= 2;
    }

    println!("Times = {}", count);
}

fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("Plural")
    } else {
        println!("Singular")
    }
}

fn change(s: &mut String) {
    print!("I have many");
    if s.ends_with("s") {
        println!("Plural");
        s.push_str("S");
    }
}

fn eat(s: String) -> bool {
    s.starts_with("b") && s.ends_with("a")
}

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    grapesLeft: i16,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapesLeft -= 1;
    }
}

enum Shot {
    Bulleseve,
    Hit(f32),
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self {
            Shot::Bulleseve => 5,
            Shot::Hit(x) if x < 3.0 => 2,
            Shot::Hit(x) => 1,
            Shot::Miss => 0,
        }
    }
}
