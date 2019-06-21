//Accessing command line arguments in Rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", args[2]);

    // for argument in args.iter() {
    //     println!("{}", argument);
    // }
}
