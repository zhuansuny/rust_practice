fn main(){
    let num = 6;
    if num < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    let a = 1;
    if a > 0{
        println!("a is positive");
    }
    else if a < 0{
        println!("a is negative");
    }
    else{
        println!("a is zero");
    }

    let condition = true;
    // 类似三元运算符
    let number = if condition{5}else{6};
    println!("The value of number is: {}", number);

}