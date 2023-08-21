#![allow(unused)]
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use rand::Rng;
#[derive(HelloMacro)]
struct Pancakes;
fn main() {
    let t = 1..=3;
    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    let other = 98_222;
    let ary = [1, 2, 3, 4];
    let str = "test";
    rand::thread_rng().gen_range(t);
    Pancakes::hello_macro();
}
