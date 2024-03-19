// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
/*
    枚举（enums）是一种定义数据类型的方式，
    它可以表示一组相关的值，并且每个值可以带有不同类型的数据
        
        基本枚举： 不带数据的枚举，用于表示一组相关的常量值
            enum Direction {
                Up,
                Down,
                Left,
                Right,
            }
            fn move_player(direction: Direction) {
                match direction {
                    Direction::Up => println!("Moving up"),
                    Direction::Down => println!("Moving down"),
                    Direction::Left => println!("Moving left"),
                    Direction::Right => println!("Moving right"),
                }
            }
        带数据的枚举： 枚举的每个变体可以带有不同类型的数据
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }
            fn process_message(message: Message) {
                match message {
                    Message::Quit => println!("Quit"),
                    Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
                    Message::Write(text) => println!("Text message: {}", text),
                    Message::ChangeColor(r, g, b) => println!("Change color to: {}, {}, {}", r, g, b),
                }
            }
        Option 枚举： Rust 的标准库提供了一个名为 Option 的枚举，
        用于表示一个值可能存在也可能不存在的情况
            fn divide(numerator: i32, denominator: i32) -> Option<i32> {
                if denominator == 0 {
                    None
                } else {
                    Some(numerator / denominator)
                }
            }
            let result = divide(10, 2);
            match result {
                Some(quotient) => println!("Quotient is {}", quotient),
                None => println!("Cannot divide by zero"),
            }

    在 Rust 中，=> 通常用于 match 表达式中，用来分隔模式（pattern）和对应的代码块（arm）
    match 表达式用于根据值的不同模式来执行不同的代码。
    每个分支由一个模式和一个 => 后面跟着的代码块组成。当匹配到某个模式时，会执行对应的代码块{}

    {:?}{:?} 占位符告诉 println! 宏使用 Debug 格式来打印 person 变量的值
*/