struct CustomSmartPointer {
    data: String,
}

// 实现drop trait一般可以允许我们在值要离开作用域时执行一些代码。
// 可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!",self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::CustomSmartPointer;

    #[test]
    fn test_customer() {
        let c =CustomSmartPointer{data: String::from("my stuff")};
        // std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数;std::mem::drop 位于 prelude
        drop(c);
        let d =CustomSmartPointer{data: String::from("other stuff")};
        println!("CustomSmartPointer created")
    }
}