fn main() {
    let mut count = 10;
    // 循环获取返回值
    let result = 'outside: loop {
        println!("again!");
        if count == 0 {
            // 从循环返回值 循环标签,在多个循环嵌套消除歧义
            break 'outside (loop {
                println!("inner!");
                count += 1;
                if count / 2 == 0 {
                    break count;
                }
            });
        }
        count -= 1;
    };
    println!("result is {result}!");
}
