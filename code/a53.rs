// A "generic" enum similar to Option
enum Maybe <T> {
    Just(T),
    Nothing,
}

use Maybe::*;

fn main() {
    let c:Maybe<i32> = Just(10);
    let d:Maybe<&str> = Just("hello");

    let e = Just(20);
    let f = Just("world");
}
