// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.



struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
    // TODO: Something goes here
}

struct ColorTupleStruct(i32,i32,i32/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
         let green = ColorClassicStruct{
            red: 0,
            green: 255,
            blue: 0,
         };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}


/*
    在 Rust 中，struct 是一种用于定义自定义数据类型的关键字
        struct Person {
            name: String,
            age: u32,
            is_student: bool,
        }
        fn main() {
            let alice = Person {
                name: "Alice".to_string(),
                age: 30,
                is_student: false,
            };
            println!("Name: {}", alice.name);
            println!("Age: {}", alice.age);
            println!("Is student: {}", alice.is_student);
        }

    除了字段之外，结构体还可以包含方法
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn is_square(&self) -> bool {
                self.width == self.height
            }
        }
        fn main() {
            let rect1 = Rectangle { width: 30, height: 50 };
            let rect2 = Rectangle { width: 40, height: 40 };

            println!("Area of rect1: {}", rect1.area());
            println!("Is rect1 a square: {}", rect1.is_square());

            println!("Area of rect2: {}", rect2.area());
            println!("Is rect2 a square: {}", rect2.is_square());
        }

            impl 关键字用于实现（implement）某个 trait 或者为某个类型实现方法
            在 Rust 中，trait 是一种定义共享行为的方法，类似于其他编程语言中的接口（interface）
            通过 trait，可以定义一个抽象的类型，描述了具体类型应该具有的方法和行为
            其他类型可以实现这个 trait，以获得相同的行为。
    
    元组结构体
    struct ColorTupleStruct(i32, i32, i32);

    #[derive(Debug)] 是一个属性（attribute），用于为结构体或枚举自动生成实现了
     Debug trait 的代码，以便可以使用 println! 宏或调试器打印这些类型的实例
*/