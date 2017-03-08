#[derive(Debug)]
enum Shape {
    Circle(u32),
    Square (u32),
    Rectangle {ht: u32, wid: u32},
}
use Shape::*;
fn main() {
    let s1 = Circle(10);
    let s2 = Square(5);
    let s3 = Rectangle {ht: 10, wid: 2};

    println!("{:?}", s3);
}

     
