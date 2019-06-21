use std::fs::File; //Import file struct 
use std::io::Read;//To perform io operations on the file

fn main() {
    let mut file = File::open("info.txt").expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops. Can not read the file.");

    println!("File Contents: \n\n{}", contents);
}
