use List::{Cons,Nil};
use std::rc::Rc;
fn main() {
    // 使用 Rc<T> 共享数据;注意 Rc<T> 只能用于单线程场景
    // 通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("a={:?};b={:?}",a,b);

    let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    //Rc::clone 引用;浅拷贝;克隆 Rc<T> 会增加引用计数
    let d =Cons(3, Rc::clone(&e));
    let f=Cons(3, Rc::clone(&e));
    println!("d={:?};f={:?};引用e: {:?}",d,f,e);

    
}
#[derive(Debug)]
enum List {
    Cons(i32,Rc<List>),
    Nil
}
