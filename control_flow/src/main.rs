fn main() {
    println!("Hello, world!");
    conditional(8i16);
    conditional(15i16);
    conditional(1i16);
    conditional(5i16);
    conditional_let(8);
    conditional_let(16);
    conditional_let(1);
    conditional_let(11);
    loop_loop(8);
    loop_loop(3);
    val_loop([1, 2, 3, 4, 5]);
    val_loop([3, 19, 18, 16, 3]);
    val_loop([1, 1, 1, 1, 1]);
}

fn conditional(num: i16) {
    if num < 5 {
        println!("{num} is less than five");
    } else if num <= 10 {
        println!("{num} is in the range of five to ten");
    } else {
        println!("{num} is over ten");
    }
}

// because if is an expression, it can be used with a let statement to assign the outcome to a
// variable:
fn conditional_let(num: u32) {
    let even_odd: &str = if num % 2 == 0 { "even" } else { "odd" };
    println!("{num} is an {even_odd} number");
}
// expressions in both code blocks in the in, else structure must both evaluate to the same type,
// otherwise the compiler will throw an error. This is because the compiler must know the type of
// the variable at compile time, and it can not do that if the 'if' block evaluates to a differnt
// type than the 'else' block.

fn loop_loop(mut num: u8) {
    println!("This is a 'loop' loop:");
    loop {
        println!("{num}");
        if num == 0 {
            break;
        }
        num -= 1;
    }
}

// loops can return values, to assign to variables or do other things:
fn val_loop(list: [u8; 5]) {
    println!("List contents:");
    for int in list {
        println!("{int}");
    }
    let mut count = list.len() - 1;
    let result: String = loop {
        if count == 0 {
            break "list contains no numbers divisible by 4".to_string();
        }
        if list[count] % 4 == 0 {
            break format!("{} is divisible by 4", list[count]);
        }
        count -= 1;
    };
    println!("{result}");
}
// You can also return from inside a loop. While break only exits the loop, return always exits
// the current function.
