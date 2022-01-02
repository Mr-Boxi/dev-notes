// 学习枚举
// 让我们看看一个需要诉诸于代码的场景，
// 来考虑为何此时使用枚举更为合适且实用。

// 1. 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

// 2. 实例化枚举（枚举值）
fn test(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// 使用枚举甚至还有更多优势。进一步考虑一下我们的 IP 地址类型，
// 目前没有一个存储实际 IP 地址 数据 的方法；只知道它是什么 类型 的。
// 考虑到已经在学习过结构体了，你可能会像示例 那样处理这个问题
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn get() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// 更加简洁方式
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));