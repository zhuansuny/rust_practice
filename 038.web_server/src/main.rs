use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs, thread, time::Duration};
use web_server::ThreadPool;
fn main() {
    // 简单的web server 使用多线程进行同时处理
    // 线程池 以channel基础进行通信、退出
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool =  ThreadPool::new(4);  //创建线程池 
    for stream in listener.incoming().take(10)  {  // 加上 take(10)  访问10次后调用 drop退出
        let stream = stream.unwrap();  
        pool.execute(|| {  
            handle_connection(stream);
        });
        // thread::spawn(||{
        //     handle_connection(stream);
        // });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";  //GET请求
    let sleep = b"GET /sleep HTTP/1.1\r\n"; //限定路由

    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep){
        // 单线程阻塞 会导致所有请求阻塞
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}