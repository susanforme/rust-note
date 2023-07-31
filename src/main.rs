use crate::garden::vegetables::Asparagus;
// 告诉编译器应该包含在src/garden.rs文件中发现的代码
pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
