fn main() {
    // slice 没有所有权
    let s =  String::from("hello world");
    let s =  first_word_index(&s);
    println!("s first word index is {}",s);
    
    let s =  String::from("hello world"); 
    let s =  first_word(&s);
    println!("s first word is {}",s);
    
    let s =  String::from("hello world"); 
    //(左闭右开) 加上=则包含
    println!("{}",&s[..5]);
    println!("{}",&s[..=5]);
    println!("{}",&s[6..]);
    println!("{}",&s[..])
    

}
// 获取第一个单词的下标
fn first_word_index(s:&String) -> usize{
    for (i,&v) in s.as_bytes().iter().enumerate() {
        if v == b' '{
            return i;
        }
        
    }
    return s.len();

}

// 获取第一个单词
fn first_word(s:&String) -> &str{
    for (i,&v) in s.as_bytes().iter().enumerate() {
        if v == b' '{
            // 部分截取 (左闭右开)
            return &s[0..i];
        }
    }
    // 全部截取
    return &s[..];

}

