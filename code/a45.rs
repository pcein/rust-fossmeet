
const N: u64 = 1000000000;

fn main() {
    let r = (0..N)
            .map(|x| x + 1)
            .fold(0, |sum, i| sum+i);

    println!("{}", r);
}

// Compile with optimizations enabled:
// rustc -O a45.rs
