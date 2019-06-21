fn main() {
    // let name = String::from("Rustacean");

    // println!("Character at index 10: {}",
    //     match name.chars().nth(10) {
    //         Some(c) => c.to_string(),
    //         None => "No character at index 10".to_string()
    //     });

    println!("Occupation is {}", match get_occupation("Rustafarian") {
        Some(o) => o,
        None => "Not found"
    })
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Rustacean" => Some("Cool dev."),
        "Snake" => Some("Snake-y"),
        _ => None
    }
}