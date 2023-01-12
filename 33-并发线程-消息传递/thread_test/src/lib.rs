use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

#[test]
fn test_mutex() {
    let m = Mutex::new(5);

    {
        let mut m = m.lock().unwrap();
        *m = 6;
        // mutex 实现了drop,会自动释放锁
    }

    println!("m = {:?}", m)
}
#[test]
fn test_mutex2() {
    let m = Mutex::new(5);

    {
        let mut m = m.lock().unwrap();
        *m = 6;
        // mutex 实现了drop,会自动释放锁
    }

    println!("m = {:?}", m)
}

/*
    原子引用计数 Arc<T>
    所幸 Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），
    所以这是一个 原子引用计数（atomically reference counted）类型。原子性是另一类这里还未涉及到的并发原语：
    请查看标准库中 std::sync::atomic 的文档来获取更多细节。目前我们只需要知道原子类就像基本类型一样可以安全的在线程间共享。

    你可能会好奇为什么不是所有的原始类型都是原子性的？为什么不是所有标准库中的类型都默认使用 Arc<T> 实现？
    原因在于线程安全带有性能惩罚，我们希望只在必要时才为此买单。如果只是在单线程中对值进行操作，原子性提供的保证并无必要，
    代码可以因此运行的更快。
*/
#[test]
fn test_mutex_arc() {
    // 多线程共享数据使用Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
