mod lib{
    pub fn m() {
        println!("条件循环语法-------------");
        // 条件表达式 if else 
        let condition = true;
        let mut number = if condition {5} else {10};
        if number < 5 {
            println!("condition was true")
        } else {
            println!("condition was false")
        }
    
        //loop 循环
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter *2;
            }
        };
        println!("The result is : {}", result);
    
    
        // while 循环
        while number != 0 {
            println!("{}!", number);
            number -= 1;
        }
    
        println!("LIFTOFF!!!");
    
        // for 循环
        let a = [1,2,3,4,5];
        for element in a.iter(){
            println!("the value is :{}", element)
        }
    
        // Range 指定一个开始的数字和一个结束的数据，Range可以生成他们之间的数字，不含结束，
        //rev方法可以反转    （1.。4）即Range
        for number in (1..4).rev() {
            println!("{}!",number);
        }
    
        println!("条件循环语法-------------");
    
    }
}


fn main() {
    lib::m();
 } 
  