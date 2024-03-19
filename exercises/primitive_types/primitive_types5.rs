// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name , age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
/*
    Destructure 解构
    解构（Destructuring）是指将复合数据类型（如元组、数组、结构体等）
    中的各个部分拆分成单独的变量

    解构数组：
        let array = [1, 2, 3];
        let [x, y, z] = array;
        println!("x is {}, y is {}, z is {}", x, y, z);
    解构结构体：
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 10, y: 20 };
        let Point { x, y } = point;
        println!("x is {}, y is {}", x, y);
    使用 _ 忽略不需要的部分：
        let tuple = (10, 20, 30);
        let (x, _, z) = tuple;
        println!("x is {}, z is {}", x, z);



*/