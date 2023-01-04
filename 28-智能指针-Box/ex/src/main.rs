use List::{Cons, Nil};
use std::ops::Deref;
fn main() {
    let b = Box::new(4);
    println!(" b= {}",b);

    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil
            ))
        )) 
    ));
    println!("{:?}",list);

    let x = 5;
    // Box::new 可以当作指针使用
    let y = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
}
#[derive(Debug)]
enum List  {
    Cons(i32,Box<List>),
    Nil, 
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
} 
// 实现Deref trait后即可使用*解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[test]
fn test_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // *y在编译的时候会被编译器解析为：*( y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); //MyBox实现deref trait，所有隐式转换成&String;String也实现了deref trait，隐式转换成&str
    hello("Lei");
    hello(&(*m)[..]);
    let n = 5;
    // hello(1);// i32没有实现deref trait，所有无法隐式转换成&str
}

fn hello(name: &str) {
    println!("hello , {}",name);
}