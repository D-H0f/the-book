fn main() {
    /* The problem with using tuples is you have to manually return all of the values you want to
     * use outside of the function's scope back from the function, which can be become overwhelming
     * if you have a lot of values you want to return ownership of. Instead, you can provide a
     * reference to the value. A reference is like a pointer in that it's an address you can follow
     * to access the data stored at that address; that data is owned by some other variable.
     * Unlike a pointer, a reference is guaranteed to point to a valid value of a particlular type
     * for the life of that reference
     * Here is how you would define and use a calculate_length function that has a reference to
     * an object as a parameter instead of taking ownership of the value: */
    let s1 = String::from("hello, fooman!");
    println!("{s1}");
    let len = calculate_length(&s1);
    println!("{s1}(is {len} characters long!)");
    /* Note:The opposite of referencing by using & is 'dereferencing', which is accomplished with
     * the dereference operator, '*'.*/
    let mut s = String::from("Hello, fooman!");
    mutable_ref(&mut s);
    println!("{s}");
    /* Mutable referenes have a big restriction: if you have a metable reference to a value, you
     * can have no other references to that value. */
    /* NOTE: a reference must exist in the same scope as its value. A function cannot return a
     * reference to a value created in the scope of the function, because as soon as the function
     * ends the value will be dropped, and the reference will point to nothing or some other data.
     * Instead, just return the value directly, so ownership is transferred to the parent scope. */
}
fn calculate_length(s: &String) -> usize {
    println!("{s}(from inside the function using a reference!)");
    s.len()
}
// just like with variables, references are not mutable by default. To modify a borrowed value, you
// have to make a mutable reference, such as in the following example:
fn mutable_ref(some_string: &mut String) {
    some_string.push_str(" How's the bar?")
}
// RULES RECAP:
/* At any given time a value can have either ONE mutable reference, or any number of immutable
* references.
 * References must always point to valid values. */
