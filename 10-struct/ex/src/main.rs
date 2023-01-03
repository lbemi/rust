struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let user2 = build_user(String::from("asd@12.com"), String::from("user_name"));
    let user1 = User {
        email: String::from("abc@123.com"),
        user_name: String::from("Niky"),
        ..user2 //复用user2中的另外两个字段
    };

    println!("{},{}", user1.active,user1.email);

    test();
}

fn build_user(email: String, user_name: String) -> User {
    User {
        user_name,
        email,
        sign_in_count: 244,
        active: false,
    }
}

// tuple strcut 定义类似tuple的struct
// 适用于想给整个tuple起名,并让它不同于其他tuple吗,而且不需要给每个元素起名
fn tuple_struct() {
    // 使用struct关键字,后边是名字,以及里面元素的类型
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let a = (1,"23",true);
}

// 空struct
fn unit_like_struct() {
    //没有任何字段的struct
    // 适用于需要在某个类型上实现某个trait,但是里面又没有想要存储的数据
    struct a{}
}

// --------计算面积------

fn area(width: u32,length: u32) ->u32 {
    width * length
}
// 使用tuple(元组)
fn area2(dim:(u32,u32)) ->u32 {
    dim.0 * dim.1
}

// 使用struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn area3(dim: &Rectangle) ->u32 {
    dim.width * dim.length
}
fn test() {
    let w = 200;
    let l = 100;
    println!("{}",area(w, l));
    let rect = (10,20);
    println!("{}",area2(rect));
    let rect2 = Rectangle{
        width: 10,
        length: 20,
    };
    println!("{}",area3(&rect2));
    println!("{:#?}", rect2)
}