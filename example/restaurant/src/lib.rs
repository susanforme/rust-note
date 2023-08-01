pub use crate::front_of_house::{hosting, serving};

// 保留声明 解析front_of_house模块 不带花括号回去解析目录
mod front_of_house;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::take_order();
}
