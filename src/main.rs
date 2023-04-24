fn main() {
    let value = fib(30);
    println!("value is {value}");
}

fn fib(num: i32) -> i32 {
    if num == 1 || num == 2 {
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}
