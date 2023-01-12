use std::{thread, time::Duration};

fn main() {

   let handle= thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread !",i);
        thread::sleep(Duration::from_millis(1));
    }
   });

   for i in 1..5 {
    println!("hi number {} from main thead!!!!!",i);
    thread::sleep(Duration::from_millis(1));
   }

   handle.join().unwrap() // 阻塞主线程,等待自己执行完后才会结束 
}

/*
move 关键字经常用于传递给 thread::spawn 的闭包，因为闭包会获取从环境中取得的值的所有权，
因此会将这些值的所有权从一个线程传送到另一个线程。
*/
#[test]
fn test() {
    let v = vec![1,2,3];
    let handle = thread::spawn( move|| {
        println!("Here's a vector: {:?}",v)
    });
    // drop(v);// v的所有权已经转移到线程中了
    handle.join().unwrap();
}