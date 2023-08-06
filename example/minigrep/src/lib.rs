#![allow(dead_code)]
#![allow(unused_variables)]
/*
 * @Author: susanforme
 * @Date: 2023-08-07 00:00:54
 * @LastEditTime: 2023-08-07 00:00:57
 * @FilePath: \minigrep\src\lib.rs
 * @Description:
 */

use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // 失败返回错误
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// dyn是 它是 “动态的”（“dynamic”）的缩写
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
