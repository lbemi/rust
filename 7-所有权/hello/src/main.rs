fn main() {
    /*  s string类型在内存中存储示意图:

      A部分(stack)   |        B部分(heap)
    --------------------------------------
     name    value  |      | index  value |
     ptr            | ==>  |   0      h   |
     len      5     |      |   1      e   |
    capacity  5     |      |   2      l   |
    ----------------|      |   3      l   |
                           |   4      o   |
                            ---------------

      ptr 指向字符串内容的内存的指针
      len 存储stack(栈)中
      capacity是指 string从操作系统中总共获得的内存的总字节数
      以上三个都是存放在stack中

      实际存放hello内容的存放在heap中
      当变量离开作用域的时候,rust会自动调用drop函数将变量使用的heap内存释放
     */

    // String 类型,
    let mut s = String::from("hello");
    s.push_str(",World");
    println!("{}", s);

    // 变量和数据交互的方式: 移动(Move)
    // s 赋值给s2的时候,s将会失效,s的在作用域结束后不会执行drop操作
    // s2 会作用域结束后调用drop释放heap(堆)内存
    // 实际上s2复制了s的A部分,类似于浅拷贝,但是s失效了,所以称作 Move
    let s2 = s;
    //下面打印报错, s已经失效了, 不可用; 即
    // println!("{}", s);
    // 深拷贝, clone,完全复制一份
    let s3 = s2.clone();

    test2();
    test3();
}
//--------------------------------------------------------------
// 返回值与作用域所有权的变化情况示例:
// 把一个变量赋值给其他变量时就会发生移动
// 当一个包含heap水的变量离开作用域时,它的值就会被drop函数清理,除非数据的所有权移动到另一个变量上了

fn test1() {
    // 将gives_ownership函数内的some_string变量所有权移动给了s1
    let s1 = gives_ownership();

    let s2 = String::from("hello");
    // 将s2的所有权移动到takes_and_gives_back函数内,即s2不再有效
    // takes_and_gives_back又将内部的a_string的所有权又移动给了s3
    let s3 = takes_and_gives_back(s2);
} // 在生命周期结束后,s1,s3会执行drop销毁,s2因为已经移动过,所以不会执行drop操作

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

// 获取a_string所有权并转移给返回值所有权
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
//--------------------------------------------------------------
//函数传值所有权

fn test2() {

    let s = String::from("hello world!");

    take_ownership(s);

    let x = 5 ;

    make_copy(x);

}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_string: i32) {
    println!("{}", some_string);
}

//--------------------------------------------------------------
//特殊情况; 想要使用s的值,但是又不是获取的它的所有权

fn test3() {

    let s1 = String::from("hello");

    let (s2, len) = calculate_lenght(s1);

    println!("The length of '{}' is {}",s2, len)

}

fn calculate_lenght(s: String) -> (String,usize) {
    let length = s.len();
    (s, length)
}