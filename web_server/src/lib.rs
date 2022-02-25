use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct PoolCreationError {
    pub error_msg: String,
    pub code: i8,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Job 将是一个有着 execute 接收到的闭包类型的 trait 对象的类型别名
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of workers in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            // 这里要将 receiver 传递到多个 Worker 实例中, 但这样是不行的
            // 因为 Rust 所提供的通道实现是多生产者, 单消费者
            // 另外从通道队列中取出任务涉及到修改 receiver, 所以这些线程需要一个能安全的共享和修改 receiver 的方式, 否则可能导致竞争状态
            // 为了在多个线程间共享所有权并允许线程修改其值, 需要使用 Arc<Mutex<T>>
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }

        match size > 0 {
            true => Ok(ThreadPool { workers, sender }),
            false => Err(PoolCreationError {
                error_msg: "`size` should more than 0".to_string(),
                code: -1,
            }),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });
        Worker { id, thread }
    }
}

impl Drop for ThreadPool {
    // 当线程池被丢弃时, 应该 join 所有线程以确保他们完成其操作
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // 我们不能调用 join, 因为每个 worker 只有一个可变借用, 而 join 会获取它所有的参数的所有权
            worker.thread.join().unwrap();
        }
    }
}
