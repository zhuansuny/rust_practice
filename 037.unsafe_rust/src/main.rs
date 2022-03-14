

fn main() {
    let mut num = 3;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    unsafe {
        *r2 = 6;
        println!("{} {}",*r1,*r2); 
    }

    unsafe {
        dangerous()
    }

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);
    assert_eq!(a,&mut [1,2,3]);
    assert_eq!(b,&mut [4,5,6]);

    unsafe{
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


// unsafe 函数 只能在unsafe代码块中调用
unsafe fn dangerous(){
    println!("is dangerous");
}

extern "C" {
    fn abs(input: i32) -> i32;
}