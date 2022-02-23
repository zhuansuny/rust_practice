fn main(){
    let mut s = String :: from("hello world");

    s.push_str("!");
    
    println!("{}",s);

    let s1 = String::from("hello");
    let s2 = s1;
    // 这时 s1 指针已经移动到 s2 ，相当于s1已经释放
    println!("{}",s1)

    


}