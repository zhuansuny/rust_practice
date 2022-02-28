use std::collections::HashMap as map; // as 重命名
mod back_of_house;
mod front_of_house;

fn main() {
    let mut map = map::new();
    map.insert("key", "value");
    let value  = map.get("key");
    println!("The value of key is {:?}",value);

    back_of_house::eat_at_home();
    front_of_house::eat_at_rustaurant();
    
}


