use std::collections::HashMap;
fn main () {
    // 初始化哈希 map
    let mut scores = HashMap::new();
    // 插入值
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Red"),20);
    
    // 获取值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("blue score is {:?}",score);

    // map所有权
   let name = String::from("name");
   let value = String::from("value");
   let mut map = HashMap::new();
   map.insert(name, value);
   //这里 name 和 value 都无效了，所有权到了map里面

   // 遍历
   for(k,v) in &scores{
       println!("{}:{}",k,v);
   }

   // 只在没有键值存在的时候插入
   scores.insert(String::from("Blue"), 22);  //直接覆盖
   let entry = scores.entry(String::from("Blue")).or_insert(50); //存在则不会进行覆盖
   println!("{:?}",entry);

   //
   let text = "hello world wonderful world";
   let mut map = HashMap::new();

   for word in text.split_whitespace() {
       let count = map.entry(word).or_insert(0);  // 返回这个键的值的一个可变引用（&mut V）
       *count += 1;
   }

   println!("{:?}",map);


}