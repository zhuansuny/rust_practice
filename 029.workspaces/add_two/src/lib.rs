#[cfg(test)]
mod tests {
    use crate::add_two;
    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}
