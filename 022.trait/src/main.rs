use core::fmt::Display;
use std::fmt::Debug;
use demo::Summary;
use demo::NewsArticle;
use demo::Tweet;
fn main() {
    // trait 类似于interface
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet :{}",tweet.summarize());
    
    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("1 new article :{}",article.summarize_author());

    notify(&article);
    notify_v2(article);
}
// trait 作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 范型实现
pub fn notify_v2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// 范型实现 （要同时实现 Summary和Display）
pub fn notify_v3<T: Summary+Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 可以用where指定
fn some_function<T, U>(t: T, u: U) -> i32
where T: Display + Clone,
U: Clone + Debug
{
    return 1;
}

// 返回trait类型
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } 
    else {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!!!!!!!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    }
}