// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility实用.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("Option is None");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);
    let mut my_empty_vec = Vec::new();
    my_empty_vec = vec![1, 2, 3, 4, 5];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    let mut c = 1;
    // Let's swap these two!
    c = value_a;
    value_a = value_b;
    value_b = c;
    println!("value a: {}; value b: {}", value_a, value_b);
}
