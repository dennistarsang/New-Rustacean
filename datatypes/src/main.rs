#![allow(dead_code)]
#![allow(unused_variables)]

//Constants
const MAXIMUM_NUMBER: u8 = 9; //variables declared in the global scope that do not change 

fn main() {
    //enums();
    //tuples();
    //structs();
    //tuple_structs();
    arrays();

}

//Enums
enum Direction {
    Up,  
    Down,
    Left,
    Right
}

fn enums() {
    //enums
    let player_direction: Direction = Direction::Right;

    match player_direction {
        Direction::Up => println!("The player is moving upwards"),
        Direction::Down => println!("The player is moving downwards"),
        Direction::Left => println!("The player is moving towards his left"),
        Direction::Right => println!("The player is moving towards his right")
    }

    //constants
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }
}

//Tuples
fn tuples() {
    let tuple1 = (20, "Rust", true, 3.9, (1, 4, 7));

    println!("{}", (tuple1.4).1);

    let tuple2 = ("Computer Science", 6.9, 99);
    let (a, b, c) = tuple2; //Destructing an assignment

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}

//Structs
struct Color {
    red: u8, //0-255
    green: u8,
    blue: u8
}
fn structs() {
    //Color: red, green, blue
    let mut bg = Color { red: 255, green: 70, blue: 15 };
    println!("{}, {}, {}", bg.red, bg.green, bg.blue);

    bg.blue = 45;

    //Pass by reference
    let blue = Color { red: 0, green: 0, blue: 255 };

    print_color(&blue);

}
fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}

//Tuple Structs
struct Colour (u8, u8, u8);

fn tuple_structs () {
    let mut red = Colour(255, 0, 0);

    red.2 = 60;

    println!("red is {}, {}, {}", red.0, red.1, red.2);
}

//arrays
fn arrays() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers2 = [2; 13];

    println!("{:?}", numbers2); // 13 elements of 2

    //numbers[0] -> 1
    //numbers[4] -> 5

    for n in numbers.iter() {
        println!("{}\n", n);
    }

    for i in 0..numbers.len() {
        println!("{}\n", i);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}

