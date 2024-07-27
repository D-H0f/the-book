fn main() {
    println!("Hello, world!");
    hello_world();
    function_with_parameter(4, 'h');
    function_with_parameter(3, 'm');
    println!(
        "'five_or_fifteen' function with input '8': {}",
        five_or_fifteen(8i32)
    );
    println!(
        "'five_or_fifteen' function with input '20': {}",
        five_or_fifteen(20i32)
    );
    let f = five_or_fifteen(2i32);
    println!("{f}");
}

// The convention for function and variable naming in rust is snake case, all lowercase letters and
// underscores.

// In rust, an expression propduces a value, and a statement are instructions to do something, but
// do not return a value.

// A function is defined with 'fn', followed by the function identifier, followed by any argumetns
// in parentheses (None, in this case) followed by curly brackets containing the functions code
// block.
fn hello_world() {
    println!("Another hello, world!");
}
// the function can be defined before or after where it is called, all that matters is that they're
// defined somewhere in the scope that can be seen by the caller.

fn function_with_parameter(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
// You can use 'return' to explicitly exit a function and return a value, but most functions return
// the last expression implicitly.
fn five_or_fifteen(num: i32) -> i32 {
    if num > 10 {
        return 15i32;
    }

    5i32
}
