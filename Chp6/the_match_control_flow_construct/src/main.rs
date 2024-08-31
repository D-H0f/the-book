// The match Control Flow Construct
/* Rust has an extremely powerful control flow construct called match that allows you to compare a
* value againsth a series of patterns and then execute code based on which pattern matches.
* Patterns can be made up of literal values, variable names, wildcards, and many other things;
* Chapter 18 covers all the different kinds of patterns and what they do. The power of match comes
* from the expressiveness of the patterns and the fact that the compiler confirms that all possible
* cases are handled.
* Think of a match expression as being like a coin-sorting machine: coins slide down a track with
* variously sized holes along it, and each coin falls through the first hole it encounters that it
* fits into. In the same way, values go through each pattern in a match, and at the first pattern
* the value 'fits', the value falls into the associated code block to be used during execution.
* Speaking of coins, let's use them as an example using mach. We can write a function that takes an
* unknown US coin and, in a similar way as the counting machine, determines which coin it is and
* returns its value in cents:
*/
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    AllTheOtherOnesImLazy,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/* We don't typically use curly brackets if the match arm code is short, as it is in the Nickle,
* Dime, and Quarter arms, but if we want to execute multiple lines of code, as with the Penny arm,
* then curly brackets are used, and the comma becomes optional.
* */
fn value_in_cents(coin: &Coin) -> u16 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Patterns That Bind To Values
        /* Another useful feature of match arms is that they can bind to the partsof the values
         * that match the pattern. This is how we can extract values out of enum variants.
         * As an examle, we will have one of the enum varians, Quarter, hold data inside it. The
         * data will be the state design of the quarter, which is signified be the UsState enum.
         * In the match arm for the Coin::Quarter variant, we will add a variable called state to
         * the pattern that matches values of the variant Coin::Quarter. When a Coin::Quarter
         * matches, the state variable will bind to the value of that quarter's state. Then we can
         * use state in the code for that arm, like so:
         * */
        Coin::Quarter(state) => {
            println!("Quarter from {state:?}");
            25
        }
    }
}

fn coin_example_code() {
    let my_change = [
        Coin::Penny,
        Coin::Quarter(UsState::Alabama),
        Coin::Penny,
        Coin::Nickel,
        Coin::Quarter(UsState::AllTheOtherOnesImLazy),
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
    ];
    let mut counter = 0usize;
    let mut my_total = 0u16;
    my_total = loop {
        if counter == my_change.len() {
            break my_total;
        }
        println!(
            "tally is currently at {}, adding {} to the tally.",
            my_total,
            value_in_cents(&my_change[counter])
        );
        my_total += value_in_cents(&my_change[counter]);
        counter += 1usize;
    };
    println!("The total value of my assorted change is {my_total} cents");
}
// Matching with Option<T>
/* In the previous section, we wanted to get the inner T value out of the Some case when using
* Option<T>; we can also handle Option<T> using match, as we did with the Coin enum. Instead of
* comparing coins, we'll compare the variants of Option<T>, but the way the match expression works
* remains the same.
* Let's say we want to write a function that takes an Option<i32> and, if there's a value inside,
* adds 1 to that value. If there isn't a value inside, the function should return the None value,
* and not attempt to perform any operations:
* */
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1i32),
    }
}
// Catch-all Patterns and the _ Placeholder
/* Using enums, we can also take special actions for a few particular values, but for all other
* values take one default action.. Imagine we're implementing a game where, if you roll a 3 on a
* dice roll, your player doesn't move, but instead gets a new fancy hat. If you roll a 7, your
* player loses a fancy hat. For all other values, your player moves that number of spaces on the
* game board. Here's a mach that implements that logic, with the result of the dice roll hadcoded
* rather than a random value, and all other logic represented by functions that just print a
* message to STDOUT:
* */
fn add_fancy_hat() {
    println!("You gained a fancy new hat!");
}
fn remove_fancy_hat() {
    println!("You lost your fancy hat :(");
}
fn move_player(num: &u8) {
    println!("Player has moved {num} spaces!");
}
fn dice_roll_logic(roll: &u8) {
    match roll {
        3u8 => add_fancy_hat(),
        7u8 => remove_fancy_hat(),
        other => move_player(other),
        /* If you don't need to use the value being matched, you can use '_' as a placeholder
         * instead, such as for a reroll() function instead of a move_player() function. If, rather
         * than having a default action being taken, you want nothing to happen, you can use '()',
         * the unit type, in place of an expression on the mach arm, like "_ => (),"
         * The '_ => ()' arm is usefull when you only want code to be executed for a subsection of
         * possible values, rather than code being executed for all possible values.
         * */
    }
}
fn dice_example_code() {
    let mut dice_roll = 9u8;
    dice_roll_logic(&dice_roll);
    dice_roll = 1;
    dice_roll_logic(&dice_roll);
    dice_roll = 3;
    dice_roll_logic(&dice_roll);
    dice_roll = 7;
    dice_roll_logic(&dice_roll);
}
fn main() {
    coin_example_code();

    let five = Some(5i32);
    let six = plus_one(five);
    if let Option::Some(num) = six {
        println!("{}", num)
    }
    let none = plus_one(None);
    if none.is_none() {
        println!("there is no value associated with this variable.")
    }
    dice_example_code();
}
/* There's more about patterns and matching that we'll cover in Chapter 18. For now, we're going to
* move on to the if let syntax, which can be useful in situations where the match expression is a
* bit wordy.
* */
