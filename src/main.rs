
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use serde_json::Value;


fn main() {
    // print
    println!("Hello, world!");
    // variable
    let x = 45;  // assigned to i32 by default
    // for mutable variables:
    let mut y = 21;
    y = 22;
    println!("x is {} and y is {}", x, y);
    // if, else if, else
    if y > 21 {
        println!("yes");
    } 
    else if y == 22 {
        println!("22");
    }
    else {
        println!("no");
    }
    // infinite loops
    let mut n = 0;
    loop {
        n += 1;
        if n == 4 {
            println!("skip");
            continue
        }
        println!("{}", n);
        if n > 10 {
            break;
        }
    }
    // while loop
    while n <= 50 {
        println!("{}", n);
        n += 1;
    }
    // for loop
    let numbers = 1..10;
    for i in numbers {
        println!("{}", i);
    }
    let names = vec!["efeferf", "cerfrfe"];
    for (i, name) in names.iter().enumerate() {
        println!("{} {}", i, name);
    }
    // enums
    enum Test {
        Qwerty
    }
    let test = Test::Qwerty;
    match test {
        Test::Qwerty => println!("Qwerty")
    }
    // constants
    const CONSTANT: u8 = 20;
    println!("{}", CONSTANT);
    // tuples
    let tup = ("dede", 21, true, (1, 2));
    println!("{} {}", tup.0, (tup.3).1);
    // tuples unpacking
    let (_a, _b, _c, _d) = tup;
    // function
    my_function(43);
    // call json fct
    json();
    // references
    let mut gg = 10;
    let ggg = &mut gg;  // mutable reference
    *ggg += 1;  // * is needed when refering to mutable references
    // structs
    struct Color {
        red: u8,
        green: u8,
        blue: u8
    }
    let color = Color { red: 255, green: 255, blue: 255 };
    println!("{} {} {}", color.red, color.green, color.blue);
    // structs tuple
    struct ColorTuple(u8, u8, u8);
    let color = ColorTuple(255, 0, 0);
    println!("{} {} {}", color.0, color.1, color.2);
    // arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", numbers[2]);
    for n in numbers.iter() {
        println!("{}", n);
    }
    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
    // implementations (methods)
    struct Rectangle {
        width: u32,
        height: u32
    }
    impl Rectangle {
        fn print_description(&self) {
            println!("Rectangle {} {}", self.width, self.height);
        }
    }
    let my_rect = Rectangle { width: 10, height: 5 };
    my_rect.print_description();
}

fn my_function(num: u32) -> bool {
    println!("{}", num);
    return true;
}

fn json() {
    let mut file = File::open("sample.json").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let data: Value = serde_json::from_str(&content).unwrap();
    println!("{}", data);
}

