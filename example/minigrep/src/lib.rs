#![allow(dead_code)]
#![allow(unused_variables)]
/*
 * @Author: susanforme
 * @Date: 2023-08-07 00:00:54
 * @LastEditTime: 2023-08-10 16:44:29
 * @FilePath: \minigrep\src\lib.rs
 * @Description:
 */

use std::error::Error;
use std::{env, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     Config { query, file_path }
    // }
    pub fn build(
        // 参数为实现了Iterator trait
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// dyn是 它是 “动态的”（“dynamic”）的缩写
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 优化
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
