
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 私有 不可显式调用
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer { // 所有成员都是公有
        Soup,  
        Salad,
    }

}


// 使用use 可以在下面的调用中省略路径
use back_of_house::Breakfast;
pub fn eat_at_home() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);
}
