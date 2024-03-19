// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

/*
    模块（modules）是一种组织代码的方式，可以帮助你将相关的功能组织在一起，
    并控制它们的可见性（私有或公有）。
    模块系统包括模块、路径（用于引用模块中的项）和 use 声明（用于引入路径）

    mod my_module {
        // 模块内容
    }

    mod my_module {
        pub fn public_function() {
            // 可以在模块外部访问
        }

        fn private_function() {
            // 只能在当前模块或其子模块中访问
        }
    }

    my_module::public_function();

    use my_module::public_function;
    fn main() {
        public_function();
    }

    // 不同文件main.rs
    mod my_module;
    fn main() {
        my_module::public_function();
    }


*/