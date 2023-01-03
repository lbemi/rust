fn main() {
    //记住字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    update_string();
    splice_string();
    index_string();
}


fn string() {
    //创建空字符串
    let s = String::new();

    let data = "inital contents";

    // to_string()可用于实现了Display trait的类型,包括字符串字面值
    let s2 = data.to_string();
    // 该方法也可直接用于字符串字面值：
    let s3 = "inital contents".to_string();

    let s4 = String::from("value");

}

fn update_string() {
    let  mut s = String::from("foo");
    // 可以通过 push_str 方法来附加字符串 slice，从而使 String 变长
    s.push_str(" bar");
    let s2 = " tow";

    // 并没有获取参数的所有权,所以s2还可使用
    s.push_str(s2);
    println!("{},{}",s,s2);

    // 使用 push 将一个字符加入 String 值中
    s.push('s');

}

fn splice_string() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world");

    // s1 被移动了，不能继续使用
    let s3 = s1 + &s2;

    println!("{}", s3);
//  ------
    let s3 = String::from("toe");
    let s1 = String::from("hello, ");
    // let s = s1 + "-" +&s2 + "-" + &s3;

    // println!("{}", s);
    //使用宏 format! 生成的代码使用引用所以不会获取任何参数的所有权。
   let s= format!("{}-{}-{}",s1,s2,s3);
    println!("{}", s);

}

fn index_string() {
    let s1 = String::from("hello");
    //字符串不支持索引
    // let h = s1[0];

    for i in s1.chars(){
        println!("{}",i);
    }
}
