use std::collections::HashMap;

fn main() {
    hash_map();
    hash_map_collect();
    hash_map_ownership();
    hash_map_visit();
    hash_map_udpate();
    hash_map_entry_or_insert();
}

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);

    println!("{:#?}", scores)
}

fn hash_map_collect() {
    let teams = vec![String::from("Bule"), String::from("Yellow")];
    let inital_scores = vec![10, 50];

    //这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构，
    // 而除非显式指定否则 Rust 无从得知你需要的类型。但是对于键和值的类型参数来说，可以
    // 使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型
    let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();

    println!("{:#?}", scores)
}

fn hash_map_ownership() {
    let filed_name = String::from("favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // 执行后,filed_name filed_value 移动失效
    // map.insert(filed_name, field_value);

    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map
    map.insert(&filed_name, &field_value);

    println!("hash_map_ownership: {:#?}, {}", map, filed_name)
}


fn hash_map_visit() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);

    scores.insert(String::from("Yellow"), 50);


    let k = scores.get(&String::from("blue"));
    match k {
        Some(item) => println!("{}", item),
        None => println!("team not exist."),
    }
    // 遍历hash map
    for (k, v ) in &scores {
        println!("{}:{}",k ,v);
    }
}

fn hash_map_udpate() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // 已存在key,则会覆盖value
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // 使用entry判断是否存在key,不存在则添加,返回这个值的可变引用
    let e = scores.entry(String::from("Yellow"));
    e.or_insert(50);

    // Blue 存在,则不会执行or_insert; key存在则返回对应value的一个可变引用
    let e = scores.entry(String::from("Blue")).or_insert(50);
    // 不会更新为50;但是返回了25的引用,下面执行了 25+33 ,所以值变成了58
    *e +=33;
    println!("{:?}", scores);
}

// 深入理解or_insert返回引用
fn hash_map_entry_or_insert() {

    // 统计单词出现频率
    let text = "hello world wonderful world wonderful";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        println!("{}",count);
        *count += 1;
    }

    println!("{:#?}", map);
}

