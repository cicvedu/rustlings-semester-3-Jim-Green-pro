// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
/*
    clone 方法用于创建一个对象的副本。
    当想要保留原始对象的所有权并同时拥有一个新的相同的对象时，可以使用 clone

    克隆简单类型的值:
        let x = 5;
        let y = x.clone();
        println!("x = {}, y = {}", x, y); // x = 5, y = 5

    克隆字符串:
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2); // s1 = Hello, s2 = Hello

    克隆向量:
        let v1 = vec![1, 2, 3];
        let v2 = v1.clone();
        println!("v1 = {:?}, v2 = {:?}", v1, v2); // v1 = [1, 2, 3], v2 = [1, 2, 3]

    在函数参数中使用 clone:
        fn modify_vec(mut vec: Vec<i32>) -> Vec<i32> {
            vec.push(4);
            vec
        }
        let original_vec = vec![1, 2, 3];
        let modified_vec = modify_vec(original_vec.clone()); // 克隆后传入
        println!("original_vec = {:?}, modified_vec = {:?}", original_vec, modified_vec); // original_vec = [1, 2, 3], modified_vec = [1, 2, 3, 4]

    克隆自定义结构体:
        #[derive(Clone)] // 实现 Clone trait
        struct Person {
            name: String,
            age: u8,
        }

        let person1 = Person { name: String::from("Alice"), age: 30 };
        let person2 = person1.clone();
        println!("person1: {:?}, person2: {:?}", person1, person2);

        如果没有 #[derive(Clone)]，你需要手动实现 Clone trait，这样的代码会更繁琐：
        struct Person {
            name: String,
            age: u8,
        }

        impl Clone for Person {
            fn clone(&self) -> Self {
                Person {
                    name: self.name.clone(),
                    age: self.age,
                }
            }
        }
*/