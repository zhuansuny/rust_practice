#[cfg(test)]  // 这个参数代表编译的时候忽略测试
mod tests {
    use super::*;
    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert!(4 == add_two(2), "not equal to 4");
    }

}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
   pub width: u32,
   pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}