mod lib;
use crate::lib::Rectangle;
use crate::lib::add_two;
fn main () {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    println!("larger.height is {} larger.width is {}", larger.height,larger.width);
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    larger.can_hold(&smaller);
    add_two(2);


}