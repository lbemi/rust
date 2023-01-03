
fn main() {
   let guess: u32 = "42".parse().expect("not a number");
   println!("{}",guess);
   
//    let x = 2.0;

//    let a = true; 

   let tup =(500,"ads", 10);
   let (x,y,z) = tup;
   println!("{},{},{}",x,y,z);
   println!("{},{},{}",tup.0,tup.1,tup.2);

   let a = [1,2,3,4];
   let b= [3;5];
   println!("a-{},b-{}",a[1],b[2]);

   another_function(10,20);

   let x = five(2);
   println!("give function return: {}",x);
} 
 
fn another_function(x: i32,y:i32) {
    println!("Another function value: {}",x);
    println!("Another function value: {}",y);
}

fn five(x: i32) -> i32 {
    5 +x
}
