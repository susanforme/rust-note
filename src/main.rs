fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        // .. 模式会忽略模式中剩余的任何没有显式匹配的值部分
        Point { x, .. } => println!("x is {x}"),
    }
    let nums = (1, 2, 3, 4, 5);
    match nums {
        (first, .., last) => {
            println!("some number is {first},{last}");
        }
    }
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}
