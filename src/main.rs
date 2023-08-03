fn main() {
    let i = 100;
    println!("{}", fib(i));
}

fn fib(i: i32) -> i32 {
    if i == 1 || i == 2 {
        return i;
    }
    return fib(i - 1) + fib(i - 2);
}
