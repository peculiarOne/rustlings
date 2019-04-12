// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    ($val:expr) => {
        {
           let mut greeting = String::from("Hello ");
           greeting.push_str(stringify!($val));
           greeting
        }
    } 
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }

    if my_macro!(56) != "Hello 56" {
        panic!("Wrong output")
    }
}
