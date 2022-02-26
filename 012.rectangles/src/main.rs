fn main() {
     // 普通方法
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {}",
        area(width1, height1)
    );

    // 使用元组重构
    let rect1 = (30, 50);
    println!("The area of the rectangle is {}", area2(rect1));

    // 使用结构体重构
    let rectangle = &Rectangle{  
        width : 30,
        height : 50,
    };
    println!("The area of the rectangle is {}",area3(rectangle));
    println!("The area of the rectangle is {}",rectangle.area());
    println!("rectangle is {:#?}", rectangle);  // {:#?}  比{:？} 多打印格式化的数据
    

    let rectangle1 = &Rectangle{  
        width : 10,
        height : 20,
    };

    println!("is rectangle can hold rectangle1 : {}",rectangle.can_hold(rectangle1));


}

fn area(width: u32,height: u32) -> u32 {
    width * height

}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 加上注解 可以通过println打印出结构体
# [derive(Debug)]
struct Rectangle {
    width:u32,  
    height:u32,
}

fn area3(rectangles: &Rectangle) -> u32 {
    rectangles.width * rectangles.height

}

// 结构体的方法定义
impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width> other.width && self.height > other.width
    }
}