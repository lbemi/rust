//trait 对象执行动态分发
/*
当对泛型使用 trait bound 时编译器所执行的单态化处理：编译器为每一个被泛型类型
参数代替的具体类型生成了函数和方法的非泛型实现。单态化产生的代码在执行 静态分发
（static dispatch）。静态分发发生于编译器在编译时就知晓调用了什么方法的时候。
这与 动态分发 （dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么
方法。在动态分发的场景下，编译器生成的代码到运行时才能确定调用了什么方法。

当使用 trait 对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于 trait
 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运
 行时使用 trait 对象中的指针来知晓需要调用哪个方法。动态分发也阻止编译器有选择
 的内联方法代码，这会相应的禁用一些优化。尽管在编写示例 17-5 和可以支持示例 
 17-9 中的代码的过程中确实获得了额外的灵活性，但仍然需要权衡取舍
*/
pub trait Draw {
    fn draw(&self);
}

/*
trait对象需要类型安全
只有对象安全（object-safe）的trait可以实现为特征对象。这里有一些复杂的规则来
实现trait的对象安全，但在实践中，只有两个相关的规则。如果一个 trait 中定义的
所有方法都符合以下规则，则该 trait 是对象安全的：
    - 返回值不是 Self
    - 没有泛型类型的参数
Self 关键字是我们在 trait 与方法上的实现的别称，trait 对象必须是对象安全的，
因为一旦使用 trait 对象，Rust 将不再知晓该实现的返回类型。如果一个 trait 
的方法返回了一个 Self 类型，但是该 trait 对象忘记了 Self 的确切类型，那么
该方法将不能使用原本的类型。当 trait 使用具体类型填充的泛型类型时也一样：具
体类型成为实现 trait 的对象的一部分，当使用 trait 对象却忘了类型是什么时，
无法知道应该用什么类型来填充泛型类型。

String 类型实现了 Clone trait，当我们在 String 的实例对象上调用 clone 
方法时，我们会得到一个 String 类型实例对象。相似地，如果我们调用 Vec<T> 实
例对象上的 clone 方法，我们会得到一个 Vec<T> 类型的实例对象。clone 方法的
标签需要知道哪个类型是 Self 类型，因为 Self 是它的返回类型。
*/
pub trait Clone {
    fn clone(&self) -> Self;
}
pub struct Screen {
    //当我们尝试编译一些违反 trait 对象的对象安全规则的代码时，我们会收到编译器的提示
    // 例如， Screen 结构体来保存一个实现了 Clone trait 而不是 Draw trait 的类型
    // 会有报错提示
    // pub components1: Vec<Box<dyn Clone>>,

    // vector 的类型是 Box<dyn Draw>，此为一个 trait 对象：它是 Box 中任何实现了 Draw trait 的类型的替身。
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // 遍历组件然后执行draw()
        for componet in self.components.iter() {
            componet.draw()
        }
    }
}

// pub struct Screen2<T> {
//     pub components: Vec<T>,
// }

// impl <T> Screen2<T> 
// where
//     T: Draw
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw()
//         }
//     }
// }

pub struct Button {
    pub weight: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //...
    }
}