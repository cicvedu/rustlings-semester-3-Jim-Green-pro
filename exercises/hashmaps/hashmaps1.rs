// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();// TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.

    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);
    basket.insert(String::from("orange"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}


/*
    用于存储键值对。哈希映射通过哈希函数将键转换为索引，
    然后在内部的数组中存储值。
    在哈希映射中查找、插入和删除操作的时间复杂度是常数级别的（O(1)）

    使用 HashMap::new() 创建一个空的哈希映射，并使用 insert 方法插入键值对：

        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Alice"), 100);
        scores.insert(String::from("Bob"), 200);


    使用 get 方法获取哈希映射中的值，返回的是一个 Option 类型，因为键可能不存在    
        
        let score = scores.get("Alice");
        match score {
            Some(value) => println!("Score of Alice: {}", value),
            None => println!("Alice not found"),
        }

    使用 insert 方法来更新值，如果键已经存在，则会覆盖原来的值；如果键不存在，则会插入新的键值对
        
        scores.insert("Alice", 150); // 更新 Alice 的分数

    使用 remove 方法删除哈希映射中的键值对：

        scores.remove("Bob");

    可以使用迭代器来遍历哈希映射中的键值对

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

*/