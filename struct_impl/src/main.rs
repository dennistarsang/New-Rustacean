struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} * {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let my_rectangle = Rectangle { width: 10, height: 10 };

    //Rectangle 10 * 5
    my_rectangle.print_description();

    // is_square
    println!("Rectangle is a square: {}", my_rectangle.is_square());

}
