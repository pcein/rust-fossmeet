
fn main() {
    let mut a = vec![10];
    let b = a.pop();

    println!("{}", b.unwrap());
    let c = a.pop();
    println!("{}", c.unwrap());
}
