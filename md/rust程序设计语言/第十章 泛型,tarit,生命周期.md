# 泛型

```rust
use std::cmp::PartialOrd;
fn main() {
    let char_list = vec!['y', 'm', 'a', 'q'];
    let number_list = vec![34, 50, 25, 100, 65];
    largest(&char_list);
    largest(&number_list);
}
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

```

## 结构体中的泛型

```rust
fn main() {
    // 必须相同
    // let wont_work = Point { x: 5, y: 4.0 };
    let float = Point { x: 1.0, y: 4.0 };
}

struct Point<T> {
    x: T,
    y: T,
}

struct Other_point<T, U> {
    x: T,
    y: U,
}

```

## 枚举中的泛型

```rust
// 标准库提供的 Option<T> 枚举
enum Option<T> {
    Some(T),
    None,
}

// 枚举也可以拥有多个泛型类型。第九章使用过的 Result 枚举定义就是一个这样的例子：
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

