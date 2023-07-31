// 定义模块
mod front_of_house {
    // 模块内还能定义模块
    pub mod hosting {
        pub fn add_waitlist() {}
        fn seat_table() {}
    }
    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
    // 同样可以定义函数
    fn test() {}
}
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_waitlist();

    // 相对路径
    front_of_house::hosting::add_waitlist();
}
