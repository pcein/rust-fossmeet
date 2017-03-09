trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.h * self.w
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

struct Rectangle {
    h: f64,
    w: f64,
}
struct Circle {
    r: f64,
}


// is "a" bigger than "b" in area?
// should work for any shape

fn is_bigger <T1:HasArea, T2:HasArea> (a: T1, b: T2) -> bool {
    a.area() > b.area()
}

fn main() {
    let r = Rectangle { h: 3.0, w: 2.0 };
    let c = Circle { r: 5.0 };
    println!("{}", is_bigger(r, c));
}
