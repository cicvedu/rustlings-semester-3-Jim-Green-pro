// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact(精确的) same elements(元素) as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.



fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain(普通) array
    let v = vec![10, 20, 30, 40];// TODO: declare your vecto(矢量) here with the macro(宏) for vectors

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
/*
    创建一个空的 Vec:
        let mut vec: Vec<i32> = Vec::new();
    使用 vec! 宏创建包含初始元素的 Vec:
        let vec = vec![1, 2, 3, 4, 5];
    向 Vec 中添加元素：
        vec.push(6); //向 Vec 中使用 push 方法可以在末尾添加元素
    获取 Vec 中的元素数量：
        let len = vec.len();
    获取 Vec 中的元素：
        let first = vec[0]; // 获取第一个元素
    使用索引遍历 Vec 中的元素：
        for i in 0..vec.len() {
            println!("{}", vec[i]);
        }
    使用 iter() 方法遍历 Vec 中的元素：
        for &num in vec.iter() {
            println!("{}", num);
        }
    修改 Vec 中的元素：
        vec[0] = 10; // 将第一个元素修改为 10
    删除 Vec 中的元素：
        let removed_element = vec.remove(1); // 删除索引为 1 的元素，并返回被删除的元素
    使用 pop() 方法删除并返回最后一个元素：
        let last_element = vec.pop(); // 删除并返回最后一个元素
    使用 contains() 方法检查是否包含特定元素：
        let contains_3 = vec.contains(&3); // 检查是否包含元素 3


*/