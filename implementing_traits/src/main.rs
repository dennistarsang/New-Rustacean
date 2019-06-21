struct Person {
    name: String,
    age: u8
}
impl ToString for Person {
    fn to_string(&self) -> String {
        format!("My name is {} and I am {}.", self.name, self.age)
    }
}

fn main() {
    let developer = Person { name: String::from("Dennis"), age: 24 };

    println!("{}", developer.to_string()); // My name is Dennis and I am 24.
}
