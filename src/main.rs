use std::thread;

fn main() {
    println!("不可变借用:");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // 不可变借用
    let only_borrows = || println!("From closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("可变借用:");
    let mut list_mut = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list_mut);
    let mut borrows_mutably = || list_mut.push(7);
    // cannot borrow `list_mut` as immutable because it is also borrowed as mutable
    // println!("After calling closure: {:?}", list_mut);
    borrows_mutably();
    println!("After calling closure: {:?}", list_mut);

    println!("所有权转移:");
    let list_move = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list_move);
    // 以便在一个新的线程而非主线程中打印 vector：
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
