/*
Rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄漏（memory leak）），
但并不是不可能。与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄漏，这意味着内存泄漏在 
Rust 被认为是内存安全的。这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的。
这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃。

避免引用循环：将 Rc<T> 变为 Weak<T>
*/

use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons,Nil};


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // 创建了循环--代码
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));


    // 如果取消最后 println! 的注释并运行程序，Rust 会尝试打印出 a 指向 b 指向 a 这样的循环直到栈溢出。

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
