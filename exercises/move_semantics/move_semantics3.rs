// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple多个 semicolons分号S necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec:&mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);

}

/*
    mut 关键字用于声明一个可变变量

    &mut 用于创建一个可变引用。可变引用允许你借用一个值并修改它，
    而不是拥有这个值。当你在函数参数中使用 &mut 时，你表明这个函数可以修改被引用的值
*/