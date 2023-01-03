use ex::NewsArticle;
use ex::Summary;
use ex::Tweet;

fn main() {

    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(" of course, as you probaably already know,people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let ariticle = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New Article available! {}", ariticle.summarize());
    test_largest();
}

fn largest<T: PartialOrd + Clone>(list:&[T]) ->T {

    let mut largest = list[0].clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

// 直接返回引用类型T
fn largest2<T: PartialOrd + Clone>(list:&[T]) -> &T {

    let mut largest = &list[0];
    for item in list {
        if item > &largest {
            largest = item;
        }
    }
    largest
}
fn test_largest() {
    let number_list = vec![1,2,3,40,12,100];
    let result = largest(&number_list);
    println!("The largest number is {}", result);


    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    println!("The largest str is {}", result);

    let result = largest2(&str_list);
    println!("The largest str is {}", result);

    let a = 3.to_string();

}