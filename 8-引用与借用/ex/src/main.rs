fn main() {
    test();
    test2();
}

// 引用 : 允许你引用某些值而不取得其所有权
fn test() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}", s1, len)
}

//借用,把引用作为函数参数这个行为叫做 借用;不可修改其引用的属性;加上mut后则可以修改
fn calculate_length(s: &mut String) -> usize {
    s.push_str(", worlkd");
    s.len()
}

fn test2() {
    let mut s1 = String::from("hello");
    // 有不可变引用后,就不能有可变引用
    let d = &s1;
    // 另一个作用域
    {
        let c = &mut s1;
    }

    let a = &mut s1;
    // 在特定的作用域内,对某一块数据,只能有一个可变的引用,防止数据竞争;b在使用时会编译报错,
    let b = &mut s1;

    a.push_str(",s");
    
    //使用b是会编译报错
    b.push_str("string");

    
    
}

// ----------------------------
// 悬空引用,即引用了一个已经被释放的变量,编译时会报错
fn test3() {
    let r = dangle();
}

// 返回的引用实际上已经被释放
fn dangle() -> &String {
    let s= String::from("hello");
    &s
}
