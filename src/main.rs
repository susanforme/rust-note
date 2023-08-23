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
    // 不发起http请求，直接使用本地的json文件
    // #[cfg(not(feature = "online-samples"))]
    // json_typegen!("Point", r#"{ "x": 1, "y": 2 }"#);

    // let str = r#"{ "x": 3, "y": 5 }"#;
    // let mut p: Point = serde_json::from_str(str).unwrap();
    // println!("deserialized = {:?}", p);
    // p.x = 4;
    // let serialized = serde_json::to_string(&p).unwrap();
    // println!("serialized = {}", serialized);

    assert_eq!(Color::Red.name(), "Red");
    assert_eq!(PubColor::Red.name(), "Red");
}

enum Test {
    A,
    B,
    C,
}
