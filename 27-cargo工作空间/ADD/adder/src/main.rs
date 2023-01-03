use add_one::add;
fn main() {
    println!("Hello, world!");
    let num = 10 ;
    println!(
        "hello, world! {} plus one is {}!",
        num,
        add(num, num)
    )
}
 