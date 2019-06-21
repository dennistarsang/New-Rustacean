fn main() {
    let mut x = 10;
    
    let items = &mut x; //Mutable reference

    *items += 1;

    println!("x is {}", items);
    println!("x is {}", x);
}
