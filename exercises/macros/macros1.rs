// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}


// macro_rules! my_macro {
//     ($($x:expr),*) => {
//         $(
//             println!("Hello Tuhin! {}", $x);
//         )*
//     };
// }

// fn main() {
//     my_macro!(5, 5, 6);
// }