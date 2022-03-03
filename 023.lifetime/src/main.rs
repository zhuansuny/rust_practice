fn main() {
    {
        let r;
        {
            let x = 5;
            r = x; // r is a reference to x 引用
        } // x 已经离开作用域

        println!("r is {}", r);
    }

    let a =  "hello";
    let b =  "hello world";

    let c = longest(a,b);
    println!("The longest string is {}", c);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
