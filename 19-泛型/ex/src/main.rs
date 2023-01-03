fn main() {
    aa();
}

// 函数中使用泛型
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

// struct 泛型使用
fn test_struct_type() {
    struct A<T> {
        x: T,
        y: T,
    }

    struct B<T, C> {
        x: T,
        y: C,
    }

    let a = A { x: 1, y: 2 };
    let b = B { x: 1, y: "as" };
}

// 枚举中使用泛型
fn enums() {
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

//方法定义中使用泛型
fn fucntion_type() {
    struct Point<T> {
        x: T,
        y: T,
    }
    // 所有类型都有的方法
    // 注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了
    impl<T> Point<T> {
        fn x(&self) -> &T{
            &self.x
        }
    }
    // 此方法只能是类型为i32时才可用
    // 另一个选择是定义方法适用于某些有限制（constraint）的泛型类型。
    // 例如，可以选择为 Point<f32> 实例实现方法，而不是为泛型 Point 实例
    impl Point<i32> {
        fn x1(&self) -> &i32 {
            &self.x
        }
    }
}

//
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn aa() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


