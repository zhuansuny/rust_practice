use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    // 新建一个通道
    // Do not communicate by sharing memory; instead, share memory by communicating.
    let (sx,rx) = mpsc:: channel();

    let handle = thread::spawn(move ||{
        sx.send("hi").unwrap();
        sx.send("hi too").unwrap();
    });

   let recv =  rx.recv().unwrap();
   let recv2 =  rx.recv().unwrap();
  
    println!("{}",recv);
    println!("{}",recv2);


    
    handle.join().unwrap();

    let (sx,rx) = mpsc::channel();
    let sx1 = sx.clone();  // 通过克隆发送者来创建多个生产者
    // let sx1 = mpsc::Sender::clone(&sx); 
    let handle = thread::spawn(move ||{
        let vals = vec!["hi","from","the","thread"];
        for val in vals {
            sx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec!["hi","from","the","thread"];
        for val in vals {
            sx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recived in rx {
        println!("{}",recived);
    }


    handle.join().unwrap();

}
