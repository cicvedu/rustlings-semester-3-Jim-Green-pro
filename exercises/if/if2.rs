// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.



pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "literally anything"{
        "baz"
    } else {
        "bar"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}


/*
    创建引用：
        let x = 5;
        let y = &x; // 创建指向 x 的引用
    使用引用：
        println!("Value of y: {}", *y); // 输出: 5
    可变引用：
        let mut x = 5;
        let y = &mut x; // 创建 x 的可变引用
    修改可变引用：使用可变引用可以修改变量的值
        *y += 1; // 修改 x 的值
        println!("New value of x: {}", x);


    函数中引用：
        传递引用给函数
            fn print_value(value: &i32) {
                println!("Value: {}", *value);
            }
            let x = 5;
            print_value(&x); // 传递 x 的引用
        返回引用：函数可以返回引用，但需要注意生命周期的问题
            fn get_value() -> &'static str {
                "Hello, Rust!"
            }
            let s = get_value(); // 获取静态字符串的引用
            println!("{}", s);
            
            生命周期
                fn main() {
                    let r;
                    {
                        let x = 5;
                        r = &x; // 错误：`x` 在这里就被销毁了
                    }
                    println!("r: {}", r);
                }
                由于 x 是在内部作用域中定义的，当作用域结束时，x 就会被销毁，
                此时 r 就指向了一个无效的内存地址。当我们尝试打印 r 的值时，
                由于引用指向无效的内存，程序会发生错误
                fn main() {
                    let r;
                    {
                        let x = 5;
                        r = &x; // 错误：`x` 在这里就被销毁了
                        println!("r: {}", r);
                    }
                }
            
            static:
                不可变静态变量
                    static HELLO: &str = "Hello, world!";
                    fn main() {
                        println!("{}", HELLO);
}               可变静态变量：
                    static mut COUNTER: u32 = 0;
                    fn main() {
                        unsafe {
                            COUNTER += 1;
                            println!("COUNTER: {}", COUNTER);
                        }
                    }可变静态变量涉及到了不安全操作，需要使用unsafe块来修改和访问它
                静态变量的生命周期：
                    static NAME: &str = "Alice";
                        fn main() {
                            let name = NAME;
                            println!("Name: {}", name);
                        }静态变量的生命周期与程序的整个运行期间相同
                静态变量与线程安全：
*/