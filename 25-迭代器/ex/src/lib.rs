#[cfg(test)]
mod tests{
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3,4];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(),Some(&1));
        println!("1-{:?}", v1_iter);
        assert_eq!(v1_iter.next(),Some(&2));
        println!("2-{:?}", v1);
        assert_eq!(v1_iter.next(),Some(&3));
        println!("3-{:?}", v1_iter);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2 ,3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total,6);
    }

    #[test]
    fn iterator_sum1() {
        let v1 = vec![1,2 ,3];
        //链式调用，生成迭代器，如果不调用消耗型适配器则迭代器不会执行；执行消耗型迭代器才会执行
        // 链式调用map（接收一个闭包）会生成一个迭代器，执行collect消耗型适配器后，会把结果收集到一个集合里；
        let v2: Vec<_> = v1.iter().map(|x| x +1).collect();

        assert_eq!(v2,vec![2,3,4]);
    }
    #[derive(Debug,PartialEq)]
    struct  Shoe {
        size:u32,
        style:String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>,shoe_size: u32) ->Vec<Shoe> {
        shoes.into_iter().filter(|x| x.size == shoe_size).collect()
    }
    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("bo0t"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(in_my_size,vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("bo0t"),
            },
        ])
    }
}