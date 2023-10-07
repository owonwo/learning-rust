pub enum Colors {
    Red,
    Blue,
    Green,
}

pub fn print_color(color: Colors) -> () {
    match color {
        Colors::Blue => println!("blue"),
        Colors::Green => println!("green"),
        Colors::Red => println!("red"),
    }
}
