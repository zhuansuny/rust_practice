fn main(){

    loop{
        println!("again!");
        break;
    }

    let mut number = 5;
     while number != 0{
        println!("{}!", number);
        number -= 1;
     }

     let arr = [10, 20, 30, 40, 50];

     for a in arr {
         println!("the value is: {}", a);
         
     }
     for (k,v) in arr.iter().enumerate() {
         println!("the index is: {} , the value is: {}", k,v);
         
     }

     // (1..5)range从1到5，不包含5（左闭右开）
     // (1..=5)range从1到5，包含5（左闭右闭）
     // rev 是反转
     
     for num in (1..5).rev(){
            println!("{}!", num);
     }
}