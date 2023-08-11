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

### 泛型代码的性能

rust会在编译时进行泛型代码的单态化

```rust
let integer = Some(5);
let float = Some(5.0);
```

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

泛型 `Option<T>` 被编译器替换为了具体的定义。

# Trait (类似interface)

*trait* 定义了某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 *trait bounds* 指定泛型是任何拥有特定行为的类型。

/src/main

```rust
// Summary必须在本地作用域
use crate::aggregator::Summary;

mod aggregator;

fn main() {
    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

```

/src/aggregator

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
pub trait DefaultSummary {
    // 带有默认实现
    fn default_summarize(&self) -> String {
        String::from("Read More")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

// 使用一个空的impl快则调用默认实现
impl DefaultSummary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

// 该参数支持任何实现了指定 trait 的类型 我们可以传递任何 NewsArticle 或 Tweet 的实例来调用 notify
// impl trait 语法
pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarize());
}

// trait bound
pub fn notify_bound<T: Summary>(item: &T) {
    println!("breaking news! {}", item.summarize());
}

//通过+ 多个trait
pub fn notify_multi(item: &(impl Summary + Display)) {}

// trait bound
pub fn notify_multi_bound<T: Summary + Display>(item: &T) {}

// 多个trait 难以阅读 使用where从句
pub fn notify_where<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Display,
    U: Clone + Debug,
{
    3
}

// 返回实现了trait 的类型
fn return_summarize(switch: bool) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    }
}

```

但是不能为外部类型实现外部 trait。例如，不能在 `aggregator` crate 中为 `Vec<T>` 实现 `Display` trait。这是因为 `Display` 和 `Vec<T>` 都定义于标准库中，它们并不位于 `aggregator` crate 本地作用域中。这个限制是被称为 **相干性**（*coherence*）的程序属性的一部分，或者更具体的说是 **孤儿规则**（*orphan rule*），其得名于不存在父类型。

### 

```rust
use std::fmt::Display;

fn main() {}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// partialOrd 允许比较 ,Display 允许打印
impl<T: Display + PartialOrd> Pair<T> {
    fn cm_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

```



# 生命周期

## 悬垂引用

借用检查器 

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

```

生命周期注解 r的叫\`a   x的叫 \`b

在编译时，Rust 比较这两个生命周期的大小，并发现 `r` 拥有生命周期 `'a`，不过它引用了一个拥有生命周期 `'b` 的对象。程序被拒绝编译，因为生命周期 `'b` 比生命周期 `'a` 要小：被引用的对象比它的引用者存在的时间更短。

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// 编译失败,函数并不知道返回的引用指向x还是y
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 编译成功,返回的函数引用存活一样久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 生命周期注解语法

```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

不同生命周期成功编译

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

尝试在string2离开作用域使用

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // 因为在string2 已经回收还在使用引用 抛错
    println!("The longest string is {}", result);
}
```

### 深入理解生命周期

```rust
// 总是返回第一个参数
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用 **没有** 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值。然而它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // 编译失败,因为返回值的生命周期与参数完全没有关联
    result.as_str()
}
```

### 结构体中的生命周期注解

结构体中能包含所有权的类型,也可以包含引用的结构体

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

```

### 生命周期省略

而且我们需要为那些使用了引用的函数或结构体指定生命周期。然而，第四章的示例 4-9 中有一个函数 它没有生命周期注解却能编译成功：

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

 Rust 引用分析的模式被称为 **生命周期省略规则**

函数或方法的参数的生命周期被称为 **输入生命周期**（*input lifetimes*），而返回值的生命周期被称为 **输出生命周期**（*output lifetimes*）

第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：`fn foo<'a>(x: &'a i32)`，有两个引用参数的函数就有两个不同的生命周期参数，`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，依此类推。

第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。

第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 `&self` 或 `&mut self`，说明是个对象的方法 (method)(译者注：这里涉及 rust 的面向对象参见 17 章)，那么所有输出生命周期参数被赋予 `self` 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。

**使用三条规则必须计算出所有引用的生命周期**

```rust
fn longest(x: &str, y: &str) -> &str {}
```

使用第一条规则

```rust
fn longest<'a,'b>(x:&' str,y:&' str)->&str{}
```

不满足第二条及第三条,但是仍然有一个参数,所以报错

### 静态生命周期

`static 其生命周期存在于整个程序期间

```rust
    let s: &'static str = "hello ";
```

### 结合泛型类型参数、trait bounds 和生命周期

```rust
fn logest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

