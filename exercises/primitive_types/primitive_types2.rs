// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types2` or use the `hint` watch subcommand
// for a hint.



fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '1';// Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

/*
    is_alphabetic() 方法可以用来检查一个字符是否是字母
    /ˌæl.fəˈbet.ɪk/α\β如果字符是字母，则返回 true

    is_numeric() 是一个方法，用于检查字符是否是数字

    在 Rust 中，双引号 "" 用于表示字符串字面量（String literals）
    而单引号 '' 则用于表示字符字面量（Character literals）
*/