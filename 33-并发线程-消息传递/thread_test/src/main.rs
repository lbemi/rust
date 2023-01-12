use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    // move 将 tx 移动到闭包中这样新建线程就拥有 tx 了。新建线程需要拥有信道的发送端以便能向信道发送消息。
    thread::spawn(move || {
        let val = String::from("hi");
        // val的所有权已经转移
        tx.send(val).unwrap();
    });
    println!("asdsd");
    // recv方法会组织当前线程执行,直到channel中有值被送来.
    let recevied  = rx.recv().unwrap();
    println!("*****");
    println!("Got: {}", recevied);

}

//发送多个值并观察接收者的等待
#[test]
fn test() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

//通过克隆发送者来创建多个生产者
#[test]
fn tes2() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}