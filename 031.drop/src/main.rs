fn main() {
   let customer =  CustomSmartPointer{
         data: String::from("hallo"),
      };
      drop(customer);  // 使用drop函数调用customer的drop函数,相当于离开作用域
      println!("end of main");
   // customer.drop(); //drop不可显示调用，在custome离开作用域时自动调用

}

struct CustomSmartPointer {
   data: String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
