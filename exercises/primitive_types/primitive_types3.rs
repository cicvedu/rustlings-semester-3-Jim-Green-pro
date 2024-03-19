// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let a = [0;1000];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
/*
    [] 通常用于两个主要目的：用于创建数组和切片

    let arr1 = [1, 2, 3, 4, 5]; // 创建一个包含 5 个元素的数组
    let arr2 = ['a', 'b', 'c']; // 创建一个包含 3 个字符元素的数组
    let arr3 = [0; 10]; // 创建一个包含 10 个元素，每个元素初始值为 0 的数组

    let arr = [10, 20, 30, 40, 50];
    let first_element = arr[0]; // 访问第一个元素，值为 10
    let third_element = arr[2]; // 访问第三个元素，值为 30

    let arr = [1, 2, 3, 4, 5];
    let slice1 = &arr[1..3]; // 创建一个包含第二个和第三个元素的切片
    let slice2 = &arr[..3]; // 创建一个包含前三个元素的切片
    let slice3 = &arr[2..]; // 创建一个包含从第三个元素到最后一个元素的切片
    切片是创建一个新的数组

    let mut arr = [1, 2, 3];
    arr[1] = 4; // 将第二个元素的值修改为 4

*/