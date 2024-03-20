// traits1.rs
//
// Time to implement实现 some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self{
        self + "Bar"
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}

/*
    // 定义一个名为Animal的trait
    trait Animal {
        // 定义一个方法，返回动物的名字
        fn name(&self) -> &str;

        // 定义一个方法，返回动物的声音
        fn make_sound(&self) -> &str;
    }

    // 实现Animal trait的Dog结构体
    struct Dog {
        name: String,
        sound: String,
    }

    // 为Dog实现Animal trait
    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }

        fn make_sound(&self) -> &str {
            &self.sound
        }
    }

    fn main() {
        let dog = Dog {
            name: String::from("Doggy"),
            sound: String::from("Woof"),
        };

        // 调用Dog实例的方法
        println!("{} says {}", dog.name(), dog.make_sound());
    }

*/