struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 元组
    let rect = (32, 10);
    // 结构体
    let rect2 = Rectangle {
        width: 32,
        height: 10,
    };
    println!("area is {}", area(rect));
    println!("area is {}", area2(&rect2));
}
fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
