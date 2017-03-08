
fn main() {
    let mut a = vec![10];
    let b = a.pop();

    match b {
        Some(x) => println!("pop: {}", x),
        None => println!("empty stack"),
    }
}
