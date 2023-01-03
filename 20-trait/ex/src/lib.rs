use std::{fmt::{format, Display, Debug}, iter::Sum};

pub trait Summary {
    // fn summarize(&self) ->String;
    //有时为 trait 中的某些或全部方法提供默认的行为，
    // 而不是在每个类型的每个实现中都定义自己的行为是很
    // 有用的。这样当为某个特定类型实现 trait 时，可以
    // 选择保留或重载每个方法的默认行为。

    fn summarize_author(&self) -> String;
    // 默认实现
    /*
    默认实现允许调用相同 trait 中的其他方法，哪怕这些方法
    没有默认实现。如此，trait 可以提供很多有用的功能而只需
    要实现指定一小部分内容。例如，我们可以定义 Summary trait，
    使其具有一个需要实现的 summarize_author 方法，然后定
    义一个 summarize 方法，此方法的默认实现调用 summarize_author 方法
    */
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) ->String {
    //     format!("{} , by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// trait作为参数传递; impl trait语法; 适用于简单情况
// item2实现了两个trait; 使用+指定多个trait
pub fn notify(item: impl Summary, item2: impl Summary + Display) ->String {
    format!("Breaking news! {}", item.summarize())
}

// trait作为参数传递;Trait Bound 语法,简洁;适用于复杂情况
pub fn notify1<T: Summary >(item: T, item2: T)  ->String{
    format!("Breaking news! {}", item.summarize())
}

// trait作为参数传递;Trait Bound 语法,简洁;使用+指定多个trait
pub fn notify2<T: Summary + Display>(item: T, item2: T) ->String{
    format!("Breaking news! {}", item.summarize())
}

// trait作为参数传递;Trait Bound 语法,简洁;使用+指定多个trait
pub fn notify3<T: Summary + Display, U: Clone + Debug>(item: T, item2: U)->String {
    format!("Breaking news! {}", item.summarize())
}

// trait作为参数传递;Trait Bound 语法,简洁;可以在方法签名后面指定where子句来指明trait
pub fn notify4<T, U>(item: T, item2: U)->String 
where
    T: Summary + Display,
    U: Clone+ Debug
{
    format!("Breaking news! {}", item.summarize())
}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}