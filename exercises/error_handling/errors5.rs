// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts概念 that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.



use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;//.parse() 是一个用于将字符串转换为其他类型的方法
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement实施 `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}

/*
    {:?} 是 Rust 中用于调试输出的格式化字符串，它会使用 Debug trait 来格式化值。
    Debug trait 是一个用于打印调试信息的 trait，
    通常通过 #[derive(Debug)] 来为自定义类型自动生成实现
        
        #[derive(Debug)]
        struct Data {
            values: [i32; 3],
        }
        fn main() {
            let data = Data { values: [1, 2, 3] };

            println!("{:?}", data); // 输出: Data { values: [1, 2, 3] }
            println!("{}", data);   // 输出: [I32(1), I32(2), I32(3)]
        }

    
    fmt::Display 是为了提供给用户可读的输出，通常用于展示给最终用户看的信息。
    {:?} 通常用于调试目的，会使用 Debug trait 来格式化输出，输出的信息更详细，更适合程序员查看。
        
        struct Point {
            x: f64,
            y: f64,
        }
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        fn main() {
            let point = Point { x: 3.5, y: 4.2 };
            println!("Display: {}", point);
        }
    
        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
        }
        fn main() {
            let point = Point { x: 3.5, y: 4.2 };
            println!("Debug: {:?}", point);
        }


    impl Trait for Type语法用于为类型实现特定的Trait。在这种情况下，XXX可以是任何你定义的Trait，
        trait MyTrait {
            fn do_something(&self);
        }
        struct MyType;
        impl MyTrait for MyType {
            fn do_something(&self) {
                println!("Doing something");
            }
        }

*/