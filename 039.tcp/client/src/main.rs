use std::{net::TcpStream, io::{Write, Read, self},str};
fn main() {
    loop {
        println!("请输入：");
        let mut  stream = TcpStream::connect("localhost:3000").unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        stream.write(input.as_bytes()).unwrap();
    
        let mut buffer = [0; 100];
        stream.read(&mut buffer).unwrap();
    
        println!("Response from server :{}",str::from_utf8(&buffer).unwrap());
        
    };

}
