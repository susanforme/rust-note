//Rng 是一个 trait
use rand::Rng;
// 导入标准库
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    // 创建不可变变量    范围表达式start..=end
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        // 创建字符串
        let mut guess = String::new();
        io::stdin()
            // 传入指针  返回一个Result枚举类型
            // 每种可能的状态称为一种 枚举成员（variant）
            .read_line(&mut guess)
            // . 方法
            .expect("Failed to read line");

        // 若没引入也可直接使用标准库
        // std::io::stdin().read_line(&mut guess).expect("error!");

        // Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };
        //match 表达式 由不同的分支(arm) 组成  一个分支包含一个模式(pattern)
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
