
fn foo() -> Vec<i32> {
    let v = vec![1, 2, 3];
    v
}

fn main() {
    let p = foo();
    println!("{:?}", p);
}
