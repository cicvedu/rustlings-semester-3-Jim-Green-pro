// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let gy = "world";
    println!("Hello {}!" , gy);
}

/*

String（字符串）类型和 &str (字符串字面量)类型:

    String 类型的可变性:
        let mut s1 = String::from("hello");
        s1.push_str(", world!");
        println!("{}", s1);  // 输出 "hello, world!"

    &str 类型的不可变性:
        let s2 = "hello";
        // s2.push_str(", world!");  // 编译错误：不能在不可变引用上调用可变方法
        println!("{}", s2);  // 输出 "hello"

    从 String 转换为 &str:
        let s3 = String::from("hello");
        let r: &str = &s3;  // 将 String 类型转换为 &str 类型
        println!("{}", r);  // 输出 "hello"

        String 类型可以被视为 &str 类型的一个视图（view）
        所以可以将其引用转换为 &str 类型的引用

*/