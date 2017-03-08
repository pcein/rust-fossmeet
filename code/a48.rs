#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square (f64),
    Rectangle {ht: f64, wid: f64},
}
use Shape::*;
fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(x) => 3.14 * x * x,
        Shape::Rectangle {ht: x, wid: y} => x * y,
    } // bug!
}
fn main() {
    let s1 = Circle(10.0);
    println!("{:?}", area(s1));
}

     
