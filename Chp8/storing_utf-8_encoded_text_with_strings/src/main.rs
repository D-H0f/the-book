fn main() {
    println!("\x1b[1;4;33mStoring UTF-8 Encoded Text with Strings\x1b[0m 

    We talked about strings in Chapter 4, but we'll look at them in more depth now. New Rustaceans 
    commonly get stuck on strings for a combination of three reasons: Rust's propensity for exposing 
    possible errors, strings being a more complicated data structure than many progroammers give them 
    credit for, and UTF-8. These factors combine in a way that can seem difficult when you're coming 
    from other programming languages.

    We discuss strings in the context of collections because strings are implemented as a collection of
    bytes, plus some methods to provide usuful functionality when those bytes are interpreted as text.
    In this section, we'll talk about the operations on String that every collections type has, such as 
    creating, updating, and reading. We'll also discuss the ways in which String is different from the 
    other collections, namely how indexing into a String is complicated by the differences between 
    how people and computers interpret String data.

\x1b[1;4;33mWhat Is a String?\x1b[0m 
    
    We'll first define what we mean by the term string. Rust has only one string type in the core 
    language, which is the string slice str that is usually seen in its borrowed form &str. In Chapter 4,
    we talked about string slices, which are references to some UTF-8 encoded string data stored 
    elsewhere. String literals, for example, are stored in the program's binary and are therefore string 
    slices.

    The String type, which is provided by Rust's standard library rather than coded into the core 
    language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to 
    \"strings\" in Rust, they might be referring to either the String or the string slice &str types, not just 
    one of those types. Although this section is largely about String, both types are used heavily in 
    Rust's standard library, and both String and string slices are UTF-8 encoded.

\x1b[1;4;33mCreating a New String\x1b[0m 

    Many of the same operations available with Vec<T> are available with String as well because 
    string is actually implemented as a wrapper around a vector of bytes with some extra guarantees,
    restrictions, and capabilities. An example of a function that works the same way with Vec<T> and 
    String is the new function to create an instance, shown here:\x1b[97m 
        let mut s = String::new();\x1b[0m 

    This line creates a new, empty string called s, into which we can then load data. Often, we'll have 
    some initial data with which we want to start the string. For that, we use the to_string method,
    which is available on any type that implements the Display trait, as string literals do. Here 
    are two examples:\x1b;97m 
        let data = \"initial contents\";
        
        let s = data.to_string();

        // the method also works on a literal directly:
        let s = \"initial contents\".to_string();\x1b[0m 

    This code creates a string containing initial contents.

    We can also use the function String::from to create a String from a string literal. The code in 
    the next example is equivilant to the last snippet, which used .to_sting():\x1b[97m 
        let s = String::from(\"initial contents\");\x1b[0m 
    
    Because strings are used for so many things, we can use many different generic APIs for strings,
    providing us with a lot of options. Some of them can seem redundant, but they all have their place.
    In this case, String::from and to_string de the same thing, so which one you choose is a matter 
    of style and readability.

    Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as 
    shown here:\x1b[97m 
        let hello = String::from(\"السلام عليكم\");
        let hello = String::from(\"Dobrý den\");
        let hello = String::from(\"Hello\");
        let hello = String::from(\"שלום\");
        let hello = String::from(\"नमस्ते\");
        let hello = String::from(\"こんにちは\");
        let hello = String::from(\"안녕하세요\");
        let hello = String::from(\"你好\");
        let hello = String::from(\"Olá\");
        let hello = String::from(\"Здравствуйте\");
        let hello = String::from(\"Hola\");\x1b[0m 
    
    All of these are valid String values.

\x1b[1;4;33mUpdating a String\x1b[0m 

    A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you 
    push more data into it. In addition, you can conveniently use the + operator or the format! macro 
    to concatenate String values.


    ");
}
