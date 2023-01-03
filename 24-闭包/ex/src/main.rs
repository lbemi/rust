use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}
//Fn 系列 trait 由标准库提供。所有的闭包都实现了
//  trait Fn、FnMut 或 FnOnce 中的一个。在 “闭包会
//  捕获其环境” 部分我们会讨论这些 trait 的区别；在这
//  个例子中可以使用 Fn trait。
struct Cacher<T>
where
    T: Fn(u32) -> u32, //约束T为闭包trait
{
    calculation: T,
    value: Option<HashMap<u32,u32>>,
    // value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match &self.value {
            // Some(v) => v,
            Some(v) => match v.get(&arg) {
                Some(u) => *u,
                None => {
                    let x = (self.calculation)(arg);
                    let mut hm = HashMap::new();
                    hm.insert(arg, x);
                    self.value = Some(hm);
                    x
                }
            },
            None => {
                let v = (self.calculation)(arg);
                let mut hm = HashMap::new();
                hm.insert(arg, v);
                self.value = Some(hm);
                v
            }
        }
    }
}
fn main() {
    let simulated_user_specified_value = 24;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout(22, 7);
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_with_different_valyes() {
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        let v3 = c.value(3);
        assert_eq!(v2, 2);
        assert_eq!(v3,3)
    }
}