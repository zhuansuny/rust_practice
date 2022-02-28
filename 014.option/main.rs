fn main(){
    // option 一般用来定义空值 
    // 一般类型没有空值概念，需要用到空值则用option
    
    // enum Option<T> { // T代表任意类型
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i8> = None;
    
    println!("some_number is none : {} absent_number is none :{}",some_number.is_none(),absent_number.is_none());
    let num = some_number.unwrap_or_default();

    println!("num is {} some_number is {}",num,some_number.unwrap_or_default());

}