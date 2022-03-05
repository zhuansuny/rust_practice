// 闭包
use std::thread;
use std::time::Duration;
// 声明一个结构体
struct Cacher<T>
where 
    T: Fn(u32)->u32 ,  // 泛型 闭包类型
{
    calculation: T,
    value: Option<u32>,

}

impl<T> Cacher<T>
where 
    T: Fn(u32)->u32,
{
    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
    fn new(cal:T) -> Cacher<T>{
        Cacher { calculation: cal, value: None }
    }
    
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 10;
    gebarate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 闭包可以捕获上下文 函数不可以
    // move 可以强制将所有权移动到闭包中
    let x = 5;
    let equal_to_x = |z:i32|  z==x;

    println!("{}",equal_to_x(4));


}

// fn simulated_expensive_calculation(intensity: u32) -> u32{
//    println!("calculation slowly..");
//    //等待两秒
//    thread::sleep(Duration::from_secs(2));
//    intensity
// }

fn gebarate_workout(intensity: u32,random_number: u32) {

    // 声明一个闭包
    // let expensive_closure =  |num|{  // 编译器可以推断出类型 也可以标注 |num: u32| -> u32
    //     println!("calculation slowly..");
    //     //等待两秒
    //     thread::sleep(Duration::from_secs(2));
    //     num+1
    // };

    let mut expensive_closure = Cacher::new( |num|{  // 编译器可以推断出类型 也可以标注 |num: u32| -> u32
        println!("calculation slowly..");
        //等待两秒
        thread::sleep(Duration::from_secs(2));
        num+1
    });

    //  使用结构体缓存闭包的结果

    // let expensive_result  = simulated_expensive_calculation(intensity); //不使用闭包

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity) );
        println!("Next, do {} pushups!", expensive_closure.value(intensity) );
    }else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            println!("Today, run for {} minutes!",expensive_closure.value(intensity) )
        }
    }


}
