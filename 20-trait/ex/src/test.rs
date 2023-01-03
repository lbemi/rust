struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y:T) ->Self {
        Self{x,y}
    }
}
/*
通过使用带有 trait bound 的泛型参数的 impl 块，
可以有条件地只为那些实现了特定 trait 的类型实现方法。
例如，示例 10-16 中的类型 Pair<T> 总是实现了 new 
方法并返回一个 Pair<T> 的实例（回忆一下第五章的 
"定义方法" 部分，Self 是一个 impl 块类型的类型别名
（type alias），在这里是 Pair<T>）。不过在下一个 
impl 块中，只有那些为 T 类型实现了 PartialOrd trait
 （来允许比较） 和 Display trait （来启用打印）的 Pair<T> 
 才会实现 cmp_display 方法：
*/
impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
} 