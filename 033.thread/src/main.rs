use std::thread;
use std::time::Duration;
fn main() {

    // 开启一个线程
   let handle = thread::spawn(|| {
        for i in 1..20  {
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程继续同步执行
    for i in 1..5  {
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("waiting spawned thread... ");

    handle.join().unwrap();  // 等待线程执行完毕


    // 线程获取上下文 通过move获取所有权
    let v = vec![1,2,3];
    let v_copy  = v.clone();

    let handle = thread::spawn( move || {
        println!("Here's a vector copy: {:?}", v_copy);
    });

    println!("Here's a vector: {:?}", v);

    handle.join().unwrap(); 

}
