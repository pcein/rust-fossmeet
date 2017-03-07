fn sqr(x: i32) -> i32 {
    let y = x * x;
    y
}
fn main() {
    let t1 = sqr(10);
    let t2:i32 = sqr(20);
    println!("sqr 10 = {}, sqr 20 ={}", t1, t2);
}
