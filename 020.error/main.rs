
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // 主动 panic
    // panic!("unknow err");
    let v = vec![1, 2, 3];
    // v[99];  //下标越限panic

    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("test.txt"){
                Ok(file) => file,
                Err(error) =>{panic!("Problem opening file: {:?}",error)},
            },
            _ =>{
                panic!("Problem opening file: {:?}",error)
            },
        },
    };
    // 简写
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); //自定义报错信息

    //传播错误

}

 // 错误返回示例
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f =  match f {  // 匹配是否错误
        Ok(file) => file, 
        Err(error) => return Err(error),  // 如果有错误，直接返回错误
    };

    let mut s = String::new();

   return match f.read_to_string(&mut s){ 
        Ok(_) => Ok(s),   // ok 返回字符串
        Err(error) => Err(error), // 错误返回错误
    };
}

// ?运算符简化
// 表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 中的值将作为整个函数的返回值
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

// 进一步简化
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
// 直接返回read_to_string的值
fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
