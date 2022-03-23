use std::{net::TcpListener, io::{Read, Write},str};
fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    println!("Running on port 3000 ");

    for stream in listener.incoming()  {
        let mut stream = stream.unwrap();
        
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        println!("Request from client: {}", str::from_utf8(&buffer).unwrap());
        stream.write(&mut buffer).unwrap();
        
    }
}
