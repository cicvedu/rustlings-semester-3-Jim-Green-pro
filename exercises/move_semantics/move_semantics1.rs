// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.



/*
    move_semantics
    在 Rust 中，"move semantics"（移动语义）是指当一个值从一个变量移动到另一个变量时，
    原来的变量将不再有效，并且新的变量获得了值的所有权。
    这与传统的拷贝语义不同，在拷贝语义中，值被复制到新变量，而原变量仍然保持有效
*/
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
