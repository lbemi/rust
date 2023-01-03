fn main() {
    test();
}
// 使用struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
// 在impl 块里定义方法
// 方法的第一个参数可以是&self,也可以获取起所有权或可变借用
// 可以有多个impl
impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) ->bool {
        self.width > other.width && self.length > other.length
    }

    // 定义关联函数: 第一个参数不是slef; 通常用于构造器
    fn square(size: u32) ->Rectangle {
        Rectangle { width: size, length: size }
    }
}

fn test() {
    let rect1 = Rectangle{
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        length: 20,
    };
    println!("{}",rect2.area());
    println!("{:#?}", rect2);
    println!("{}",rect1.can_hold(&rect2));
    // 关联函数调用
    let s = Rectangle::square(10);
    println!("{:#?}", s)
}