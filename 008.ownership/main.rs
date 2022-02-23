fn main(){
    let mut s = String :: from("hello world");

    s.push_str("!");
    
    println!("{}",s);

    let s1 = String::from("hello");
    let s2 = s1;
    // 这时 s1 指针已经移动到 s2 ，相当于s1已经释放
    // println!("{}",s1)
    println!("{}",s2);
    
    // 需要复制一份使用，需使用clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1= {} s2 = {}",s1,s2);

    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s

    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1); // s1无效

    println!("The length of '{}' is {}.", s2, len);

}

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn calculate_length(s:String) -> (String,usize){
    let length  =  s.len();
    (s,length)

}