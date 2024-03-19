// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a < b { b } else { a }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
/*
    cfg 属性不仅限于 test，它可以用于多种不同的配置选项，允许你根据不同的条件编译不同的代码块。这些条件可以是目标操作系统、目标架构、自定义标志等。例如：
    #[cfg(target_os = "windows")]: 只在目标操作系统为 Windows 时编译代码。
    #[cfg(target_arch = "x86_64")]: 只在目标架构为 x86_64 时编译代码。
    #[cfg(feature = "my_feature")]: 只在启用了名为 my_feature 的功能标志时编译代码。
 
    mod 一种组织代码的模块
    主文件直接mod XXXXX;  XXXXX::XXX()
    可嵌套
    Rust 中用于创建对象和组织相关数据和行为的主要工具是结构体（struct）和特质（trait）
    使用 use 关键字可以将模块或模块中的项引入当前作用域，这样可以减少代码中的重复路径

 */