// https://youtu.be/OLVVeRF09Dw?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL

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
}

fn my_function(num: u32) -> bool {
    println!("{}", num);
    return true;
}