struct CustomSmartPointer {
    data: String,
}

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
        drop(c);
        let d =CustomSmartPointer{data: String::from("other stuff")};
        println!("CustomSmartPointer created")
    }
}