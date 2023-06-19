fn main() {
    let str = String::from("hello world");
    let mut str2 = String::from("hello world");
    let len = calculate_len(&str);
    // 把对象的引用作为参数而不是所有权的转移,使用值
    println!("len is {}! str is {}!", len, str);
    change(&mut str2);
    println!("str2 is {}!", str2);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    // 不能修改借用的值
    s.push_str("!");
}
