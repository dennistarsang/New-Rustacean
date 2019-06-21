#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    //if_statement();
    //rust_loop();
    //while_loop();
    for_loop();
}

//if statements
fn if_statement() {
    let n = 49;

    if n == 40 {
        println!("Rust");
    }
    else if n > 45 {
        println!("It is greater than 45")
    }
    else {
        println!("It was not 40");
    }

}

//Loop keyword
fn rust_loop() {
    let mut n = 0;

    loop {
        n += 1;

        if n == 7 {
            continue;
        }

        if n > 10 {
            break;
        }

        println!("The value of n is {}", n);
        } 
    }

// while loop
fn while_loop () {
    let mut n = 1;

    while n <= 50 {
        //if n is a multiple of 5
        if n % 5 == 0 {
            println!("n = {}", n);
        }
        println!("n is = {}", n);

        n += 1;
    }
}

//for loop
fn for_loop() {
    let numbers = 30..51;
    let animals = vec!["hyena", "koala", "kangaroo", "dog"];

    for i in numbers {
        println!("The no is {}", i); //i holds the current element
    }

    for a in animals.iter() { // Use .iter() in vectors to prevent the ownership from being passed to the for loop
        println!("The animal you are looking for is a {}.", a);
    }

    for (index, a) in animals.iter().enumerate() { // Use .enumerate() to find the index of vector elements
        println!("The index is {} and the animal is a {}.", index, a);
    }
}


