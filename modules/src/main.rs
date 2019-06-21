mod sang {
    fn chicken() {
        println!("Chicken!");
    }
    pub fn print_message() {
        println!("How is it going?");

        chicken();
    }

    pub mod water {
        pub fn is_wet() {
            println!("Water is wet");
        }
    }
}

fn main() {
    sang::print_message();
    sang::water::is_wet();
}
