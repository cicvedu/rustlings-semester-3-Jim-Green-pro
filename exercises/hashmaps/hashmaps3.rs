// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        /*
            其他用法
            XXX 是一个不可变引用（&mut i32 类型），
            它指向 HashMap 中 "Alice" 对应的值。
            当使用 entry 方法和 or_insert 方法时，
            如果键（在这个例子中是 "Alice"）已经存在于 HashMap 中，
            or_insert 方法会返回一个指向该键对应值的可变引用。
            修改 alice_score 的值时，实际上是在直接修改 HashMap 中 "Alice" 对应的值

            使用 entry 方法和 or_insert 方法获取 HashMap 中的值时，
            返回的是一个可变引用（&mut 类型）指向该值。
            即使 HashMap 本身没有被标记为可变（mut），返回的引用仍然是可变的。
            允许通过这个引用来修改 HashMap 中的值，而不需要将整个 HashMap 声明为可变的
         */
        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();  
        //.sort() 方法对向量 keys 中的元素进行排序。由于元素是 &String 类型的引用，
        //排序会基于字符串的字典序进行
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}

/*
    XXX.lines() 是一个 Rust 中常用的字符串方法，它返回一个迭代器，
    该迭代器产生字符串 XXX中的每一行（以换行符 \n 分隔）

    .split(',') 是一个字符串方法，用于将字符串按照指定的分隔符 ,
     进行分割，并返回一个迭代器，该迭代器产生分割后的子字符串
     .collect() 是一个通用的方法，用于将迭代器的元素收集到一个集合中，
     例如 Vec、HashMap
        
        // 分割逗号分隔的字符串成单独的部分并收集到一个 Vec 中
        let csv_data = "name,age,city";
        let fields: Vec<&str> = csv_data.split(',').collect();
        println!("{:?}", fields); // 输出结果: ["name", "age", "city"]

        // 分割由连字符分隔的字符串成单词并收集到一个 Vec 中
        let hyphenated_words = "rust-is-awesome";
        let parts: Vec<&str> = hyphenated_words.split('-').collect();
        println!("{:?}", parts); // 输出结果: ["rust", "is", "awesome"]
    
    .parse() 方法用于将字符串解析为相应的类型，比如将字符串解析为整数、浮点数等。
    在 Rust 中，.parse() 返回一个 Result 枚举类型，表示解析的结果可能是成功（Ok）或失败（Err）。
    如果解析成功，它会返回解析后的值，否则会返回一个解析错误。

    .unwrap() 方法用于获取 Result 类型的值，如果是 Ok 则返回其中的值，
    如果是 Err 则会导致程序崩溃并显示错误消息。在实际应用中，
    最好使用 match 或者 ? 运算符来处理 Result 类型，以便更好地处理可能出现的错误情况。

    

*/

/*
    string和str的区别
        存储方式:
        String: 是一个可增长的、堆分配的、UTF-8 编码的字符串类型。
        它可以修改，比如增加、删除或修改其中的字符。
        str: 通常以 &str 的形式出现，是一个不可变的、固定长度的字符串切片，
        通常指向字符串字面量或 String 中的一部分。
        所有权:
        String: 拥有其内容的所有权，当 String 离开作用域时，其内容会被自动释放。
        str: 通常没有所有权，它是借用的，不能直接创建一个 str 类型的变量，
        只能通过借用 String 或其他 str 来使用。

         1: 创建和修改字符串：
            // String
            let mut s = String::from("Hello");
            s.push_str(", world!");  // 修改 String
            println!("{}", s);       // 输出: Hello, world!
            // str
            let s = "Hello, world!";  // 创建一个 &str
            // s.push_str(", Rust!"); // 错误: 不能修改 &str
            println!("{}", s);        // 输出: Hello, world!

         2: 切片和借用
            // String
            let s = String::from("Hello, world!");
            let slice = &s[7..12];    // 切片 String，得到 &str
            println!("{}", slice);    // 输出: world

            // str
            let s = "Hello, world!";
            let slice = &s[7..12];    // 切片 &str，得到 &str
            println!("{}", slice);    // 输出: world
        
        3: 传递给函数
            // 使用 String
            fn greet(name: String) {
                println!("Hello, {}!", name);
            }
            let name = String::from("Alice");
            greet(name);
            // println!("{}", name); // 错误: value borrowed here after move
            // 使用 &str
            fn greet(name: &str) {
                println!("Hello, {}!", name);
            }
            let name = "Alice";
            greet(name);
            println!("{}", name); // 正确: &str 是不可变引用

        4: 从函数返回字符串：
            // 返回 String
            fn create_greeting() -> String {
                String::from("Hello, world!")
            }
            let greeting = create_greeting();
            println!("{}", greeting);
            // 返回 &str
            // fn create_greeting() -> &str {
            //     "Hello, world!" // 错误: 不能返回局部变量的引用
            // }

    


    &String &str区别：
        自动解引用：
            fn print_str(s: &str) {
                println!("{}", s);
            }
            fn main() {
                let string = String::from("Hello, world!");

                print_str(&string); // 自动解引用: &String -> &str
            }
        显式解引用：
            fn print_string(s: &String) {
                println!("{}", s);
            }
            fn main() {
                let string = String::from("Hello, world!");
                let str_slice: &str = &string; // 显式地将 &String 转换为 &str

                print_string(&string); // 正确: 传递 &String
                // print_string(str_slice); // 错误: 不能将 &str 转换为 &String
            }
            
            fn print_string(s: &String) {
                println!("{}", s);
            }
            fn print_str(s: &str) {
                println!("{}", s);
            }
            fn main() {
                let str_slice = "Hello, world!";
                // print_string(&str_slice); // 错误: 不能将 &str 转换为 &String
                print_str(&str_slice); // 正确: 传递 &str
            }


*/