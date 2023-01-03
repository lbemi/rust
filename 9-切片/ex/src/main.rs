fn main() {
    let  s = String::from("Hello, world!");
let word_index = first_word(&s[..]);

    println!("{}", word_index);

    let my_string_literal = "hello world";
    let word_index = first_word(&my_string_literal);

    // s已经借用为不可变引用,所以s.clear()会报错
    // s.clear();

    println!("{}", word_index);

    // slice()
}

fn first_word(s: &str) -> &str  {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn slice() {
    let  s = String::from("Hello, world!");

    let hello = &s[..6];
    let world = &s[6..];
    let whole = &s[..];
    println!("{}{}- {}", hello ,world, whole);

    // 字符串字面值时切片,被直接存储在二进制程序中,不可变
    let  s2 = "Hello world";
}

fn slice2() {
    let a = [1, 2,3 ,4, 5];
    let slice = &a[2..4];
}