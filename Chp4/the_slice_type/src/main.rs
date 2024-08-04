fn main() {
    // Slices let you reference a contigueous sequence of elements in a collection, rather than the
    // whole collection.
    // A slice is a kind of reference, so it does not have ownership.
    let s = String::from("hello fooman, this is a string");
    let word: &str = first_word(&s);
    println!("{s}");
    println!("first word is {}", word);
    let secondword: &str = second_word(&s);
    println!("second word is {}", secondword);
    let s = String::from("this string");
    let word: &str = second_word(&s);
    println!("{word}");
    let s = String::from("reallyreallyreallylongword!!!");
    println!("{}", second_word(&s));
    let s = String::from(" string starting with a space");
    println!("{}", second_word(&s)); // can't handle specific edge cases, but for the purposes of
                                     // this educational project there is no reason to spend time enabling
                                     // it to handle these cases.
    print_binary("hello");
}
fn first_word(s: &String) -> &str {
    // This converts the string into an array of bytes to iterate over
    let bytes = s.as_bytes();
    /* bytes.iter() creates an iterator over the array of bytes, and .enumerate() wraps the results
     * of .iter() and returns each element as part of a tuple, containing the index and result.
     * because iter returns a reference to the element, you must use '&item' in the pattern */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn second_word(s: &String) -> &str {
    // returns the second word if one exists, otherwise returns the first word
    let bytes = s.as_bytes();
    let mut start: usize = 0;
    /* realized that variable shadowing (re-assigning an immutable variable) doesn't carry to
     * outide of a block's scope, so the 'let start: usize = i' in the 'if' block would be dropped
     * as soon as the if's scope ends, making it useless. Figured out to just make 'start' mutable*/
    let end: usize = 0;
    println!("{}, len: {}", s, s.len());
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' || item == b'.' {
            if start == 0usize {
                println!("start var assigned val: {}", i);
                start = i + 1;
            } else if end == 0usize {
                println!("return option 1 (second word found in string with >2 words)");
                return &s[start..i];
            }
        } else if i + 1 == s.len() && start != 0usize {
            println!("return option 2 (only two words in string)");
            return &s[start..];
        }
    }
    println!("return option 3 (only one word in string)");
    &s[..]
}
fn print_binary(s: &str) {
    let bytes = s.as_bytes();
    for &item in bytes.iter() {
        match char::from_u32(item as u32) {
            Some(c) => println!("{:b}: {}", item, c),
            None => println!("{:b} is not a valid char", item),
        }
    }
    for &item in bytes.iter() {
        print!("{:b} ", item);
    }
}
