extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "Sang sang a song";

    println!("Found match? {}", re.is_match(text));
}
