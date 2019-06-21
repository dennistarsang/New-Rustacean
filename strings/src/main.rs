fn main() {
    let mut my_string = String::from("I am a cold blooded Rustacean. What's up friend?");

    println!("{}", my_string);

    //Length
    println!("The length of my string is {}", my_string.len());

    // Is empty?
    println!("Is my string empty? : {}", my_string.is_empty());

    // Split using whitespace
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    //Does it contain another string?
    println!("Does it contain 'Rustacean'? : {}", my_string.contains("Rustacean"));

    //Appending a string
    my_string.push_str("\nWelcome home.");
    println!("{}", my_string);
}
