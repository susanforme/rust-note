use std::{
  sync::{
      mpsc::{self, Receiver},
      Arc, Mutex,
  },
  thread,
};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
      // 需要闭包一直循环，向信道的接收端请求任务，并在得到任务时执行他们
      let thread = thread::spawn(move || loop {
          let message = receiver.lock().unwrap().recv();
          // 当 let 语句结束时任何表达式中等号右侧使用的临时值都会立即被丢弃
          match message {
              Ok(job) => {
                  println!("Worker {id} got a job; executing.");
                  job();
              }
              // 显式退出循环
              Err(_) => {
                  println!("Worker {id} disconnected; shutting down.");
              }
          }
      });
      // 不会丢弃  recv这里不会释放锁 job()
      // job() 调用期间锁一直持续，这也意味着其他的 worker 无法接受任务。
      //   while let Ok(job) = receiver.lock().unwrap().recv() {
      //     println!("Worker {id} got a job; executing.");
      //     job();
      // }
      Worker {
          id,
          thread: Some(thread),
      }
  }
}

impl ThreadPool {
  /// 创建线程池。
  ///
  /// 线程池中线程的数量。
  ///
  /// # Panics
  ///
  /// `new` 函数在 size 为 0 时会 panic。
  pub fn new(size: usize) -> Self {
      assert!(size > 0);
      // 新建一个信道
      let (sender, receiver) = mpsc::channel();
      let receiver = Arc::new(Mutex::new(receiver));
      let mut workers = Vec::with_capacity(size);
      for id in 0..size {
          workers.push(Worker::new(id, Arc::clone(&receiver)))
      }
      ThreadPool {
          workers,
          sender: Some(sender),
      }
  }
  // pub fn build(size: usize) -> Result<Self, ()> {
  //     if size == 0 {
  //         return Err(());
  //     }
  //     Ok(ThreadPool)
  // }
  pub fn execute<F>(&self, f: F)
  where
      F: FnOnce() + Send + 'static,
  {
      let job = Box::new(f);
      self.sender.as_ref().unwrap().send(job).unwrap()
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
      // 显式丢弃
      drop(self.sender.take());
      for worker in &mut self.workers {
          println!("Shutting down worker {}", worker.id);
          // take移出
          if let Some(thread) = worker.thread.take() {
              thread.join().unwrap();
          }
      }
  }
}
