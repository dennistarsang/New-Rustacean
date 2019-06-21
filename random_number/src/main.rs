extern crate rand;
use rand::Rng; // A trait of rand crate

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11);

    println!("Random Number: {}", random_number);
}
