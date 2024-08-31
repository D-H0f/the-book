// Concise Control Flow With if let
/* The if let syntax lets you combine if and let into a less verbose way to handle values that
* match one pattern while ignoring the rest. Consider the following example program that matches on
* an Option<u8> value in the config_max variable but only wants to execute code if the value is the
* Some variant:
* */

fn config_max_match_example() {
    let config_max = Option::Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    /* If the value is Some, we print out the value in the Some variant by binding the value to the
     * variable max in the pattern. We don't want to do anything with the None value. To satisfy the
     * match expresson, we have to add `_ => ()` after processing just one variant, which is annoyng
     * boilerplate code to add.
     * Instead, we could write this in a shorter way using if let. The following code behaves the
     * same as the match in the previous example.
     * */
}

fn config_max_if_let_example() {
    let config_max = Option::Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max} (this time using if let)");
    }
}

/* The syntax if let takes a pattern and an expression separated by an equal sign. It works the
* same way as a match, where the expression is given to the match and the pattern is its first arm.
* In this case, the pattern is Some(max), and the max binds to the value inside the Some. We can
* then use max in the body of the if let block in the same way we used max in the corresponding
* match arm. The code in the if let block doesn't run if the value doesn't match the pattern.
* Using if let means less typing, less indentation, and less boilerplate code. However, you lose
* the exhaustive checking that match enforces. Choosng between match and if let depends on what
* you're doing in your particular situation and whether gaining conciseness is an appropriate
* trade-off for losing exhaustive checking.
* We can include an else with an if let. The block of code that goes with the else is the same as
* the block of code that would go with the _ case in the match expression.
* */
fn config_max_if_let_else_example() {
    let config_max = Option::Some(3u8);
    fn quick_find(config: &Option<u8>) {
        if let Option::Some(max) = config {
            println!("max has been set to {max}");
        } else {
            println!("The given variable has a value of None");
        }
    }
    quick_find(&config_max);
    quick_find(&Option::None);
}
fn main() {
    config_max_match_example();
    config_max_if_let_example();
    config_max_if_let_else_example();
}
