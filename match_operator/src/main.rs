fn main() {
    let number = 16;

    match number {
        1 => println!("It is one."),
        2 => println!("It is two."),
        10 | 11 => println!("It is either ten or eleven."),
        12...20 => println!("It is greater than 11."),
        _ => println!("It doesn't match.")
    }

    let name = "Sang";

    match name {
        "Dennis" => println!("No. This is his first name"),
        "Kipkemei" => println!("No. This is his middle name"),
        "Sang" => println!("Yes! This is his surname"),
        _ => println!("Wrong name entered.")
    }
}
