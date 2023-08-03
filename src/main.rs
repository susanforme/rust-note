use std::cmp::max;
use std::collections::HashMap;
fn main() {
    let vec = vec![3, 3, 7, 4, 9, 9, 111, 1, 1, 1];
    let mut map = HashMap::new();
    let mut count = 0;
    let mut key = 0;

    for i in 0..vec.len() {
        let count = map.entry(vec[i]).or_insert(0);
        *count += 1;
    }
    for k in map.keys() {
        let max_num = max(count, map.get(k).copied().unwrap_or(0));
        if max_num != count {
            key = *k;
            count = max_num;
        }
    }
    println!("众数是{},重复{}次!", &key, count);
}
