use std::ops::Deref;

//智能指针（smart pointers）是一类数据结构,类似指针
use List::*;
fn main() {
    //Box<T>，用于在堆上分配值 
    let b = Box::new(5);
    println!("b = {}",b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("list = {:?}",list);


    let x = 5;
    let y = Box::new(x);  // y是一个智能指针，指向一个值5 相当于&x
    assert_eq!(5,x);  // & 引用  * 解引用
    assert_eq!(5,*y);

    let mybox = MyBox::new(5);
    assert_eq!(5,*mybox);
    println!("mybox = {:?}",mybox);


}
#[derive(Debug)] 
enum List {
    // Cons(i32, List),  //这样会报错，因为是个循环，不可获取具体的大小
    Cons(i32, Box<List>),  // Box相当于只存储了指针，指向的是堆上的内存 ，大小固定
    Nil,
}


// 自定义智能指针
// 需要实现Deref trait 可以使用解引用 *
#[derive(Debug)]
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T>Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
