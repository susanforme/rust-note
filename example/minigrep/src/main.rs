#![allow(dead_code)]
#![allow(unused_variables)]
use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    // let Config {
    //     query: _,
    //     file_path,
    // } = Config::new(&args);
    // 调用宏输出
    // dbg!(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // 打印到标准错误流
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // if let检查run是否返回Err
    if let Err(e) = run(config) {
        // 打印到标准错误流
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // 运行 cargo run > output.txt  现在我们看到了屏幕上的错误信息，同时 output.txt 里什么也没有，这正是命令行程序所期望的行为。
    // cargo run -- to poem.txt > output.txt 我们并不会在终端看到任何输出，同时 output.txt 将会包含其结果：
}
