use std::collections::HashMap;
fn main() {
    // 所有的键必须是相同类型，值也必须都是相同类型。
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 40);

    // 访问map中的值
    let team_name = String::from("blue");
    // get方法返回Option<&V> copied获取Option<T> 接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零。
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("the {} team score is {}!", team_name, score);

    // 遍历 注意是随机顺序遍历
    for (key, value) in &scores {
        println!("{key}:{value}");
    }
}
