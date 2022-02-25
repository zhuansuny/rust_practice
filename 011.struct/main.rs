fn main () {
    // 创建User结构体实例
    let mut user = User {
        email: String::from("xxx@163.com"),
        user_name: String::from("XXX"),
        active:true,
        age:18,
    };
    user.age = 19;
    println!("name: {} email:{} age:{} active:{}",user.user_name,user.email,user.age,user.active);
    
    // 构建函数
    let mut user = build_user(String::from("dog"), String::from("xxx.@ee.com"), 17);
    user.age = 20;

    println!("name: {} email:{} age:{} active:{}",user.user_name,user.email,user.age,user.active);
    
    // 没有命名字段的元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(1, 2, 3);
    let origin = Point(4, 5, 6);
    
    println!("color: {} point:{} ",black.2,origin.2);




}


struct User {
    user_name: String,
    email:    String,
    active:   bool,
    age:      u32,  
}

fn build_user(user_name: String,email: String,age: u32) -> User{
    User {
        user_name,   //可以省略字段名
        email: email,
        active:true,
        age:age,
    }

}

