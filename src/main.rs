struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 指定限制
impl Point<f64> {
    fn only_support_i32(&self) -> f64 {
        &self.x + &self.y
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    let float = Point { x: 3.5, y: 4.5 };
    // 只能在f64情况下使用
    float.only_support_i32();
    let t = p.x();
    println!("x is {t}");
}
