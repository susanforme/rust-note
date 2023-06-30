#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 方法  implementation 缩写
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 32,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 22,
        height: 10,
    };
    let rect3 = Rectangle::square(22);
    println!("rect1 can hole rect2? {}!", rec1.can_hold(&rect2));
    println!("rect2 can hole rect3? {}!", rec1.can_hold(&rect3));
}
