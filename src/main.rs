#![allow(unused)]

use rust_note::Post;

fn main() {
    // 不再完全遵守面向对象的状态模式
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
