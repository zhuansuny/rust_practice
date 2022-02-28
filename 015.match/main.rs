fn main(){
     // match 类型与其它语言中的 switch
    let coin = Coin::Penny;
    println!("the coin value is {}",value_is_cents(coin)) ;
    
    // 匹配option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six value is {}  none value is {}",six.unwrap_or_default(),none.unwrap_or_default()) ;

    // 匹配是 穷尽的 可以使用 _通配符列举其它情况
    let some_u8_value:u8 = 0; 
    match some_u8_value{
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("other"),
    };

    // if let 简化版 match 
    if let Some(5) = six{
        println!("five")
    } else {
        println!("not five")
    };
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_is_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,

    }
}

fn plus_one(x :Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}



