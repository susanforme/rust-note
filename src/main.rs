#![allow(unused)]

use crate::a::b::PubColor;
mod a;
mod marco;

enum_str! {
    enum Color {
        Red,
        Green,
        Blue,
    }
}

fn main() {
    assert_eq!(Color::Red.name(), "Red");
    assert_eq!(PubColor::Red.name(), "Red");
}

enum Test {
    A,
    B,
    C,
}
