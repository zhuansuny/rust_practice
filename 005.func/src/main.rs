fn main() {
    let  a = 22;
    let  b = 22;
    another_function(a,b);

    println!("five : {}" ,five());

    println!("add : {}" ,add(a,b))
}

fn another_function(a: i32, b:i32){
    println!("a + b = {}",a+b);
}
fn five() ->  i32{
    5
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}