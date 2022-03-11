use std::{sync::{Mutex, Arc}, thread};
fn main() {
    // 互斥器 锁 mutex 不需要手动unlock
    let m = Mutex::new(5);  // 创建一个i32的互斥器
    {
        // 在这个区域内，可以访问互斥器
        let mut num = m.lock().unwrap();  // 获取互斥器的锁
        *num = 6;  // 改变互斥器的值
    }

    println!("m= {:?}",m);

    // 原子引用计数 Arc<T> 
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10  {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);    
    };

    for handle in handles  {
        handle.join().unwrap();
    }
    println!("Result= {:?}",counter.lock().unwrap());
}
