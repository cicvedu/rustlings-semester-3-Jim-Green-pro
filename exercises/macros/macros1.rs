// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

/*
    其中 () => {} 是一种宏的匹配模式和展开式的组合。
    这个模式表示宏不接受任何参数，并且在展开时将替换为大括号中的代码

    macro_rules! greet {
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
    }
    fn main() {
        greet!("Rustaceans"); // 输出: Hello, Rustaceans!
    }

    接受可变参数的宏:
    macro_rules! print_numbers {
        ($($number:expr),*) => {
            $(
                println!("{}", $number);
            )*
        };
    }
    fn main() {
        print_numbers!(1, 2, 3, 4, 5); // 依次输出: 1 2 3 4 5
    }

*/