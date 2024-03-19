// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

/*
    .to_string() 方法
    是 Rust 中将其他类型转换为 String 类型的一种常用方法。
    可以用于任何实现了 ToString trait 的类型
        将整数转换为字符串：
            let num = 42;
            let num_str = num.to_string();
            println!("The string representation of num is: {}", num_str);

        将浮点数转换为字符串：
            let float = 3.14;
            let float_str = float.to_string();
            println!("The string representation of float is: {}", float_str);

        将布尔值转换为字符串：
            let boolean = true;
            let boolean_str = boolean.to_string();
            println!("The string representation of boolean is: {}", boolean_str);
        将字符转换为字符串：
            let character = 'a';
            let character_str = character.to_string();
            println!("The string representation of character is: {}", character_str);

    data.chars() 将 data 转换为一个字符迭代器。
    .last() 获取迭代器中的最后一个元素，返回一个 Option<char> 类型。
    .unwrap() 从 Option<char> 中提取字符，如果 Option 是 None（即字符串为空），它会引发 panic

        在 Rust 中，Option 是一个枚举（enum），用于表示一个值可能存在也可能不存在的情况

    
    函数体内&
        1. 借用变量的引用
            fn print_value(value: &i32) {
                println!("The value is: {}", value);
                //在 Rust 中，当你使用 {} 打印一个引用时，
                //Rust 会自动解引用引用类型，以便打印出引用所指向的值
                //在 Rust 中，迭代器也会自动解引用
            }
            fn main() {
                let num = 42;
                print_value(&num); // 传递 num 的引用
            }
        2. 使用引用修改变量的值
            fn add_one(value: &mut i32) {
                *value += 1; // 解引用并修改值
            }
            fn main() {
                let mut num = 42;
                add_one(&mut num); // 传递 num 的可变引用
                println!("The new value is: {}", num);
            }
        3. 使用引用访问结构体字段
            struct Person {
                name: String,
                age: u32,
            }

            fn print_person_name(person: &Person) {
                println!("The person's name is: {}", person.name);
            }

            fn main() {
                let person = Person {
                    name: "Alice".to_string(),
                    age: 30,
                };
                print_person_name(&person); // 传递 person 的引用
            }
        4. 使用引用避免所有权转移
            fn print_length(s: &String) {
                println!("The length of the string is: {}", s.len());
            }
            fn main() {
                let s = "Hello".to_string();
                print_length(&s); // 传递 s 的引用
                println!("The original string is: {}", s); // s 仍然有效
            }


    在 main 函数中没有使用 mut 关键字来声明 num 为可变变量，
    但是在将 num 传递给 string_uppercase 函数时，
    它被移动（因为它的类型是 String，是一个拥有所有权的类型），
    所以在函数内部可以对它进行修改
        fn add_one(mut value: i32) -> i32 {
            value += 1;
            value
        }

        fn main() {
            let num = 42;
            let new_num = add_one(num); // 将 num 的值传递给函数，并获取返回值
            println!("Original num: {}", num); // num 仍然是不可变的
            println!("New num: {}", new_num); // new_num 是修改后的值
        }

*/