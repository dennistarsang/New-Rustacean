fn main() {
    let mut x = 10;

    {
        let x = 15;

    }

    println!("x = {}", x);

    //Changing a data type from int to String
    let x = "X is a string";

    println!("x = {}", x);

    //Changing the data type from String to Bool
    let x = true;

    println!("x = {}", x);
}


