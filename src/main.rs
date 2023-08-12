use std::ops::Deref;
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // 底层调用等同于  *(y.deref())
    // Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作，
    // 外边的普通解引用仍为必须的原因在于所有权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权。
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // 访问元组结构体的第一个元素
        &self.0
    }
}
