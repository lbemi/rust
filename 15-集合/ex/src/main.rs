fn main() {
    vector();
}
//vector 允许我们一个挨着一个地储存一系列数量可变的值
fn vector() {

    // 创建一个空的vector,需要指定类型
    let v: Vec<i32> = Vec::new();

    // 使用初始值创建vector
    let v2= vec![1,2,3,4,5];

    // 不指定类型,后续执行添加会自动推导出变量类型
    let mut v3 = Vec::new();
    v3.push(1);
    println!("{:#?}",v3);

    // 索引超出程序会panic
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    // 索引超出程序会执行None分支,不会panic
    match v2.get(2) {
        Some(t) => println!("The third element is {}", t),
        None => println!("There is no third element."),
    }
    // 不可变借用
    let first = &v2[0];
    // 由于 vector 的工作方式：在 vector 的结尾增加新元素时，
    // 在没有足够空间将所有元素依次相邻存放的情况下，可能会要求
    // 分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素
    // 的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况
    // v2.push(9);

    println!("The first element is {}", first);

    //我们也可以遍历可变 vector 的每一个元素的可变引用以便能改变
    //他们。 for 循环会给每一个元素加 50：
    let mut v4= vec![1,2,3,4,5];
    for i in &mut v4 {
        *i += 50;
    }
    // for循环便利vecotr
    for i in &v4 {
        println!("{}",i)
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
// 常规vector只能储存相同类型的值,但是可以通过枚举类型变体实现存储不同类型的值
fn vector2() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];
}