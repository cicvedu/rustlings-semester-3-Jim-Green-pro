// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.


/*
    该函数签名指定了返回一个字符串引用 &str，但是没有指定这个引用的生命周期。
    由于 Rust 的引用规则，编译器无法确定返回的引用的生命周期，
    因为它无法确定参数 x 和 y 的生命周期，
    这两个参数又是函数参数 x: &str 和 y: &str 的衍生物。
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
