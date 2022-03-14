use std::{thread, sync::{mpsc, Arc, Mutex}};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);  
        let (sender,receiver) = mpsc::channel();  // 创建一个Message channel 
        let receiver = Arc::new(Mutex::new(receiver)); // 接受者 创建原子锁
        let mut workers = Vec::with_capacity(size);  // 线程工作区

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));  //创建线程通过channel 进行通信
        }

        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self, f:F) 
    where 
    F:FnOnce()+Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();  // 发送者发送 Message

    }

}

// 优雅退出线程
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            // 发送线程退出消息
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");
        for worker in &mut self.workers  {
            println!("Shutting down worker {}",worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            
        }
        
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; excuting.",id);
                    job();
                },
                Message::Terminate =>{
                    println!("Worker {} was told to terminate",id);
                    break;
                }
                
            }
        });
        Worker { id, thread: Some(thread)}

    }
}
