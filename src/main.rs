fn main() {
    let roll = 9;
    match roll {
        1 => {
            println!("bingo your value is {}!", 1);
        }
        // 通配必须放最后,顺序匹配
        other => move_player(other),
        // 占位符 最后一个分支中明确地忽略了其他的值
        // _ => (),
    }
}

fn move_player(num: i32) {
    println!("other value is {}!", num);
}
