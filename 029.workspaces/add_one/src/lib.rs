#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn it_works() {
        assert_eq!(add_one(2), 3);
    }
}
use rand;
pub fn add_one(x: i32) -> i32 {
    let a: u8 = rand::random();
    println!("{}", a);
    x + 1
}
