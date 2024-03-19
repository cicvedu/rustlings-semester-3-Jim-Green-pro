// vecs2.rs
//
// A Vec of even偶数 numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.乘以2
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



/*
    XXX.iter_mut() 返回一个迭代器，该迭代器会产生 v 中每个元素的可变引用

    iter() 方法：iter() 方法用于创建一个迭代器，
    该迭代器允许您按顺序访问集合（比如 Vec、数组等）
    中的每个元素。这个迭代器是只读的，也就是说，您可以访问元素但不能修改它们
        let v = vec![1, 2, 3];
        for element in v.iter() {
            println!("{}", element);
        }
    
    map() 方法是一个高阶函数，它接受一个闭包作为参数，
    并对迭代器中的每个元素执行闭包中定义的操作，然后返回一个新的迭代器
        let v = vec![1, 2, 3];
        let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
        println!("{:?}", doubled);
    

    闭包，闭包是一个类似函数的代码块，可以在需要时被调用。
    在 Rust 中，闭包可以捕获其环境中的变量，并对这些变量进行操作
    || 参数列表
    {} 代码块
        带参数的闭包：
            let square = |x: i32| {
                x * x
            };
            println!("{}", square(5));
        不带参数的闭包：
            let say_hello = || {
                "Hello, world!"
            };
            println!("{}", say_hello());

    collect() 是一个用于收集迭代器中元素的方法，将它们放入一个集合类型中
    在 Rust 中，collect() 方法通常与迭代器的 map() 方法一起使用，
    以便对元素进行转换后收集到一个新的集合中

*/

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|&element| {
        // TODO: Do the same thing as above - but instead of mutating改变(mutate mutatant) the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
/*
    (1..)：这是一个 range，表示从 1 开始的无限序列。
    在 Rust 中，.. 表示一个不包含结尾值的范围
    在 Rust 中，(1..) 是一个从 1 开始的无限范围（range）表达式，
    它代表一个从 1 开始的无限序列。但是，它本身并不会占用无限空间。
    这是因为 (1..) 是一个惰性求值（lazy evaluation）的迭代器，
    它只在需要生成下一个元素时才会计算该元素
    
    filter(|x| x % 2 == 0)：filter 是一个迭代器适配器方法，
    用于过滤出满足条件的元素

    take(5)：take 是另一个迭代器适配器方法，用于取出指定数量的元素

    collect()：collect 是一个方法，用于将迭代器中的元素收集到一个集合中

    v.clone() 创建了 v 的一个副本，
    这样 vec_loop 函数可以使用这个副本而不会影响原始的 v 向量

    
*/