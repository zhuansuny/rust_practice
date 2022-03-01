fn main() {
    // vector先入后出 类似栈 但可以通过索引来获取值
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];
    v.push(4);
    v.push(5);
    v.push(6);
    println!("v value is {:?}",v);
    println!("v top value is {:?}", v.pop());
    println!("v top value is {:?}", v.pop());
    println!("v value is {:?}",v);
    let value = v[0];    // 两种方式都可以 获取索引对应的值
    let value = v.get(0);
    match value {
        Some(i) => {
            println!("value is some")
        },
        None => { // 索引超过最大值为none
            println!("value is none")
        }
    };
    println!("v value is {:?}",v);
    
    // 遍历
    
    for i in &v{ // 加上 &  v才不会交出所有权
        println!("value is {}",i);
        
    }

    println!("v value is {:?}",v);
    
    
    //使用枚举来存储多种类型
    #[derive(Debug)]
    enum SpreadDheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = [
        SpreadDheetCell::Int(3),
        SpreadDheetCell::Float(0.33),
        SpreadDheetCell::Text(String::from("333")),
        ];

    println!("row value is {:?}",row);
        


}