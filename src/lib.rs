pub trait Draw {
    // 抽象
    fn draw(&self);
}
pub struct Screen {
    // trait 对象 Box<dyn Draw> 实现了Draw trait的都可以
    pub components: Vec<Box<dyn Draw>>,
}

pub struct OtherScreen<T: Draw> {
    pub components: Vec<T>,
}

// 使用trait bound
impl<T> OtherScreen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// 可以各自实现
impl Draw for Button {
    fn draw(&self) {
        // draw
    }
}
