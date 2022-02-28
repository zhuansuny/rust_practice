fn main() {
    println!("Hello, world!");
    let x = 2.0; //f64
    let y:f32 = 2.0; //f32
    let z = 0.1;
 
    println!("X:{} Y:{} Z:{}",x,y,z);

    let tup:(i32,f64,u8) = (100,1.4,3);

    let (x,y,z) = tup;
   

    println!("X:{} Y:{} Z:{}",tup.0,tup.1,tup.2);
    println!("X:{} Y:{} Z:{}",x,y,z);
    
    
    let b:bool = true;
    
    println!("B:{}",b);
    
    let b:&str = "hahaahahah"; //string
    
    println!("B:{}",b);
    
    let b:char = 'b'; // char
    
    println!("B:{}",b);
    
    let guess:u32 = "42".parse().expect("Not a number!");
    println!("B:{}",guess);

    let emo:char = '😀';
    println!("emo:{}",emo);

    let arr = [1,2,3,4,5,];
    println!("arr:{:?}",arr[0]);
 
    
    
}
