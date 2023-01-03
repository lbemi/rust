fn main() {
    let four = IpAddrkind::V4(0,0,0,0);
    let six = IpAddrkind::V6(String::from("::1"));
    route(four);
    route(six);
    route(IpAddrkind::V6(String::from("::1")));

    let home = IpAddr{
        kind: IpAddrkind::V4(127,0,0,1),
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr{
        kind: IpAddrkind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    let a = IpAddrkind1::V4;
    println!("{:?}",a);
}
#[derive(Debug)]
enum IpAddrkind1 {
    V4,
    V6,
}
enum IpAddrkind {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn route(ip_kind: IpAddrkind) {}


struct IpAddr {
    kind: IpAddrkind,
    address: String,
}

// 将数据附加到枚举的变体中
// 优点:不需要额外使用struct;每个变体可以拥有不同类型及关联的数据量
enum Message {
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32, i32),
}
impl Message {
    fn call(&self) {
        
    }
}

fn test_message() {
    let q = Message::Quit;
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::Write(String::from("sss"));
    let c = Message::ChangeColor(0, 255, 255);

    c.call();
    
}

// option 枚举类型
// enum Option<T> {
//     Some(T),
//     None,
// }
// 包含在prlude(预导入模块)中,可以直接使用;
fn test_option() {

    let somne_number = Some(5);
    let some_string = Some("ssa");
    // OPtion<T>便是这个变量的值可能为空,如果想要使用则必须转换为T
    let absent_number:Option<i32> = None;


  
    let x: i8= 5;
    let y: Option<i8> = Some(5);

    // Option<T> 中的T和T是不同的类型,所有下面语句报错;要想使用必须转换为T
    // let sum = x + y;
}