// cargo run who poem.txt > output.txt  > 保存到文件中不会输出到控制台
use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    };
}
