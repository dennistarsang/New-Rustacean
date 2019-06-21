fn main() {
    //Replace method
    {
        let my_string = String::from("Rust is fantastic!");
        println!("{}", my_string);
        println!("After Replace: {}", my_string.replace("fantastic", "awesome"));
    }

    //Lines method
    {
        let my_string = String::from("I love these crates:\ndiesel\nserde.");

        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }

    //Split method
    {
        let my_string = String::from("Jared+Richard+Gilfoyle+Dinesh+are+SV+actors.");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);
    }

    //Trim method
    {
        let my_string = String::from("   I Love Rust! 🦀     \n\r");

        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    //Chars
    {
        let my_string = String::from("Dennis on GitHub");
        println!("{}", my_string);

        //Get character at index
        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4")
        }
    }
}
