fn main() {
    {
        // Each value in Rust has an owner.
        // There can only be one owner at a time.
        // When the owner goes out of scope, the value will be dropped.
        let s: &str = "hello"; // s is valid from this point forward
        println!("{s}");
    } // this scope is over, s is no longer valid
    {
        let mut s = String::from("hello");

        println!("{s}");
        s.push_str(", world!"); // push_str() appends a literal to a string
        println!("{s}");
    }
    {
        let x: u8 = 5;
        let y = x;
        /* These two previous lines do what you would expect, bind 5u8 to x, then copy x's value to
        y */
        println!("{}{}", x, y);
        let s1 = String::from("hello");
        println!("{s1})");
        let mut s2 = s1;
        /*
        because String is a heap-allocated type, this works differently than the previous example.
        A String variable is made up of three parts: A pointer to the heap memory that holds the contents
        of the string, a length, and a capacity. This meta-data is stored on the stack.
        When we assign s1 to s2, the String data is copied, meaning the the pointer, length, and capacity
        that are on the stack. The data on the heap that the pointer refers to is not copied.
        rather than shallow copying, like in python, rust does something called a move. moving the value of
        s1 to s2 invalidates the s1 variable, and it will not be able to be used.
        This means rust will never automatically create a 'deep' copy of data, and an automatic copying can
        be assumed to be inexpensive in terms of runtime performance */
        println!("s2 = {s2}");
        s2.push_str(", world");
        println!("s2 = {s2}");
    }
    {
        // if we do want to deeply copy the heap data of the String, not just the stack data, we
        // can use a common method called clone:
        let s1 = String::from("hello");
        let mut s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
        s2.push_str(", world!");
        println!("updated s2: {s2}");
        /* The reason this wasn't true with the int example is because types such as integers
         * that have a known size at compile time are stored entirely on the stack, so copies of the
         * actual values are quick to make
         * If a type implements the Copy trait, variables that use it do not move, but are trivially
         * copied, making them still valid after assignment to another variable
         * types that implement Copy:
         * * All the integer types,
         * * All the floating-point types,
         * * The char type,
         * * Tuples, if they only contain types that also implement Copy*/
    }
    ownership_and_functions();
    return_values_and_scope();
}
fn ownership_and_functions() {
    let s = String::from("hello, fooman");
    takes_ownership(s); // s's value is MOVEd to the function
                        // now, just like when assigning before, s can no longer be used

    let x: u8 = 5;
    makes_copy(x); // u8 is Copy, so its value will be copied into the function rather than
                   // moved, without having to explicitly use '.clone()'
    println!("{} is still in scope!", x);
    // now with a string:
    let s = String::from("hello, fooman");
    copies_string(s.clone());
    println!("{s} is still in scope");
}
fn takes_ownership(some_string: String) {
    println!("{some_string} will be out of scope and unusable once the function ends");
}
fn makes_copy(some_integer: u8) {
    println!("{some_integer}");
}
fn copies_string(some_string: String) {
    println!("copied \"{}\"", some_string);
}
fn return_values_and_scope() {
    // Returning values can also transfer ownership.
    let s1 = gives_ownership(); // gives_ownership moves its value into s1
    println!("{s1}");

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which moves its
    println!("{s3}"); // return value to s3
}
fn gives_ownership() -> String {
    String::from("yours")
}
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
/* The ownership of a variable follows the same pattern every time: assigning a value to another
* variable moves it. When a variable that includes data on the heap goes out of scope, the value
* will be cleaned up by drop unless ownership of the data has been moved to another variable
* While this works, it can be tedious to return ownership of every value we want to use.
* To solve this, Rust lets us return multiple values using a tuple, or, even better, using
* a reference, which is in the next sub-chapter! */
