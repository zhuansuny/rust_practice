mod front_of_house { //模块
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist")
        }
    }
}

pub fn eat_at_rustaurant() {
    // absolute path
    front_of_house::hosting::add_to_waitlist();
    println!("eat at rustaurant");

}
