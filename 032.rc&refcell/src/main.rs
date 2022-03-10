use crate::List::*;
use crate::ListV2::*;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum List {
    Cons(i32,Rc<List>),
    Nil,
    
}

#[derive(Debug)]
enum ListV2 {
    ConsV2(Rc<RefCell<i32>>, Rc<ListV2>),
    Nil,
}

fn main() {
    //RC 可以多个引用
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));  // 记录强引用计数
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("{:#?},{:#?}",b,c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));



    //RefCell内部可变性，允许即使在有不可变引用时也可以改变
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsV2(Rc::clone(&value), Rc::new(ListV2::Nil)));

    let b = ConsV2(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ConsV2(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    //树
    let leaf = Rc::new(Node {
        value: 3,
        left: RefCell::new(vec![]),
        right: RefCell::new(vec![]),
    });
    println!("{} {} {}",leaf.value,leaf.left.borrow().len(),leaf.right.borrow().len());

    let branch = Rc::new( Node{
        value: 5,
        left: RefCell::new(vec![Rc::clone(&leaf)]),
        right: RefCell::new(vec![]),
    });

    println!("branch = {:#?}", branch);

    


}

// 树形结构
#[derive(Debug)]
struct Node {
    value: i32,
    left:RefCell<Vec<Rc<Node>>>,
    right:RefCell<Vec<Rc<Node>>>,
}