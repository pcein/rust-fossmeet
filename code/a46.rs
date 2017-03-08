enum Color {
    Red,
    Green,
    Blue,
}
use Color::*;
fn main() {
    let c = Red;
    match c {
        Red => println!("color is Red!"),
        Green => println!("color is Green!"),
        Blue => println!("color is Blue!")
    }
}
