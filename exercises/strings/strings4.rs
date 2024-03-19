// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
/*
    字符串字面值（如 "blue"）是 &str 类型的，所以它们被传递给 string_slice 函数。
    使用 to_string()、String::from()、to_owned() 或 into() 方法创建的值是 String 类型的，所以它们被传递给 string 函数。
    使用 format! 宏创建的值也是 String 类型的。
    使用 &String::from("abc")[0..1] 创建的值是 &str 类型的切片，所以它被传递给 string_slice 函数。
    使用 trim()、replace() 或 to_lowercase() 方法处理的值都是 String 类型的，所以它们被传递给 string 函数。
*/