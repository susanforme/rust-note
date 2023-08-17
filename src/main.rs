#![allow(unused)]
use std::{slice, thread, time::Duration};

// 全局变量也被称为static变量
// 访问不可变静态变量是安全的

static HELLO_WORLD: &str = "hello world";

// 访问和修改可变静态变量都是 不安全 的
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    // 可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针，。
    let mut num = 5;
    let r1 = &num as *const i32;
    // 可变裸指针
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
    let address = 0x012345usize;
    let r = address as *mut i32;
    unsafe {
        // 任意访问必定崩溃
        // println!("r is {}", *r);
        // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    }
    // 必须在unsafe调用
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    thread::sleep(Duration::from_secs(100));
}

// 不安全的函数
unsafe fn dangerous() {}

// 封装不安全代码
fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    // as_mut_ptr 方法访问 slice 的裸指针
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    // 编译失败多次可变借用
    // (&mut values[..mid], &mut values[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 调用外部代码
extern "C" {
    fn abs(input: i32) -> i32;
}

// 从其它语言调用 Rust 函数
// 注解来告诉 Rust 编译器不要 mangle 此函数的名称。
// Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时，
// 这会增加用于其他编译过程的额外信息，
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
// extern 的使用无需 unsafe。

// 实现不安全 trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}
