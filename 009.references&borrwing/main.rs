fn main() {
    let s1 = String :: from("hello");

    // 以对象的引用作为参数，不会获取s1的所有权 不可修改
    let len = calculate_length(&s1);
    println!("'{}' length is {}", s1,len);

    //需要修改则可变引用，但同一作用域不可存在两个可变引用
    let mut s1 = String :: from("hello");
    
    {
        let r2 = &mut s1;
    } //这个时候已经取消了引用

    let r1 = &mut s1;
    r1.push_str(" world");

    println!("s1 is {}",s1);


    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

}

fn calculate_length(s :&String) -> usize{
    // s 相当于借用了所有权 ，但不能修改

    // s.push_str("world"); // 不能修改
    // println!("{}",s);


    return s.len()
}