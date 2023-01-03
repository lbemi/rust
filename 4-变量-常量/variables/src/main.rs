
// 定义常量,常量默认是大写
const MAX_OPINTS:u32 = 100_000;

fn main() {
    println!("Hello, world!");

    //声明一个变量使用 mut,
    let mut a = 1;
    a = a + 3;
    // 声明变量
    let  x =" 5"; 
    // shadowing(隐藏),二次定义变量新变量会覆盖老的变量,切类型可以和之前的不同
    let x = x.len();

    println!("{}, {}, {}",a,x,MAX_OPINTS)
}
 