use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
fn main() {
    let value = Rc::new(RefCell::new(5));
    // bc 都可以引用 a
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    // 这里使用了第五章讨论的自动解引用功能
    *value.borrow_mut() += 10;

    // 都可以拥有修改后的值15
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
