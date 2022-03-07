// ‘///’ 代表文档注释 cargo doc 可以生产文档




//! # Art
//! 
//! A library for modeling artistic concepts.
//! 
pub use kinds::PrimaryColor;  //使用 pub use 可以将深层次的类型导出 提前到外部 以便其他文件使用
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: SecondaryColor) -> String {
        format!("{:?}-{:?}", c1, c2)
    }
}


/// Adds one to the number given.
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(6, cargo::add_one(5));
/// ```


pub fn add_one(x: i32) -> i32 {
    x+1

}