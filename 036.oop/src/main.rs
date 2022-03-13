use oop::*;
use oop::Summary;
mod gui;
use gui::*;
mod blog;
use blog::*;
mod blog_optimize;

fn main() {
    // ====== 面向对象编程
    
    // ====== 封装 通过结构体来封装数据，私有属性，公有方法
    // 公有属性：结构体中的属性，可以被外部访问
    let mut average = AveragedCollection::new(vec![1,2,3,4], 1.5);
    average.add(5);
    println!("average: {}", average.average());


    //继承（Inheritance）是一个很多编程语言都提供的机制，一个对象可以定义为继承另一个对象的定义
    //Rust 使用默认 trait 方法实现来进行共享，
    //任何实现 trait 的类型都可以使用  trait 方法而无须进一步实现。这类似于父类有一个方法的实现，而通过继承子类也拥有这个方法的实现。
    //当实现  trait 时也可以选择覆盖 summarize 的默认实现，这类似于子类覆盖从父类继承的方法实现。

    let a =  average.summarize();
    average.print(a);

    let article = NewsArticle {};
    average.print(article.summarize());


    // ====== 多态
    // 子类型可以用于父类型被使用的地方 类似于上面的继承
    
    println(average);
    println(article);



    // ====== 一个GUI例子
    // 通过遍历列表并调用每一个项目的 draw 方法来将其绘制到屏幕上

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();



    // ===== 面向对象 -- 状态模式
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");
    println!("{}",post.content());

    post.request_review();  // 进行审批请求
    println!("{}",post.content());  // 内容为空
    
    post.approve();   // 审核成功，可以显示内容
    println!("{}",post.content());



    //===== 根据rust的特性来优化blog
    let mut post = blog_optimize::Post::new();

    post.add_text("hello world");

    let post = post.request_review();

    let post = post.approve();
    println!("{}",post.content());

}

fn println<T:Summary>(summary:T)  {
    summary.print(summary.summarize()+"\n");
}
    
