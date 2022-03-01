fn main() {
    // String 字符串 UTF-8编码
    //新建空字符串
    let mut s = String::new();

    // 使用to_string 创建字符串
    let data = "initial contents";
    let s = data.to_string();
    println!("s : {}",s);

    let s = "initial contents".to_string();
    println!("s : {}",s);

    // 使用String::from 创建字符串

    let data = String::from("initial contents");

    // 增加字符串
    let mut s = String::from("hello");
    s.push_str(" world");

    // +运算符
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 +&s2;   //s1交出了所有权不可使用  s2可以
    println!("s3 is {}",s3);

    // format! 拼接
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("!");

    let s3 = format!("{} {} {}", s1, s2, s3);
    println!("s3 is {}",s3);

    // String 不可使用索引
    // let s1 = String::from("hello");
    // let h = s1[0];

    // slice
    let mut hello = "hello world";
    let s = &hello[0..=5];
    println!("s3 is {}",s);

    // 遍历
    for b in "abc".bytes(){
        println!("{}",b);
    };

}