fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(c))
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),//绑定值模式
}

fn value_in_cents(coin: Coin) ->u8 {
    // match模式匹配
    match coin{
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel =>5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // macth匹配必须穷举所有可能性
    match x {
        None => None,
        Some(i) => Some(i +1),
    }

  
}

fn test() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn test2() {
    // macth匹配必须穷举所有可能性
    // 但是太多可能性可以使用_ => ()来忽略,且必须放在最后面
    let v = 0u8;
    match v {
        1 => println!("one"),
        2 => println!("two"),
        7 => println!("seven"),
        _ => (),
    }
}

// if let 只关注需要匹配的条件,其他则不关心;放弃了穷举的可能
fn if_let() {

    let v = Some(0u8);
    if let Some(3) = v {
        println!("three")
    } else {
        println!("others")
    }
}