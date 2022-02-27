fn main (){
    // 枚举实例
    let four = IpAddrKind :: V4;
    let six = IpAddrKind :: V6;
    // 各个枚举都可以调用函数
    route(four);
    route(six);

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home is {:?} loopback is {:?}",home,loopback);

}
// 定义一个枚举类型
# [derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

// 数据直接放进每一个枚举成员
# [derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}
// let home = IpAddr::V4(String::from("127.0.0.1"));
// let loopback = IpAddr::V6(String::from("::1"));

// 给每个枚举成员不同的类型
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

# [derive(Debug)]
struct IpAddr{
    kind : IpAddrKind,
    address: String,

}

fn route(kind: IpAddrKind){
    println!("the kind is {:?}",kind);
}

