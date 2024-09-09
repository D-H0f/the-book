fn main() {
    println!("\x1b[1;4;33mStoring Lists of Values with Vectors\x1b[0m 

    The first collection type we'll look at is Vec<T>, also known as a vector. Vectors allow you to store
    more than one value in a single data structure that puts all the values next to each other in memory.
    Vectors can only store values of the same type. They are useful when you have a list of items, such 
    as the lines of text in a file or the prices of items in a shopping cart.\n");

    println!("\x1b[1;4;33mCreating a New Vector\x1b[0m");
    // To create a new empty vector, we call the Vec::new function, as shown here:
    let v: Vec<i32> = Vec::new();

    println!("
    To create a new empty vector, we call the vec::new functoin, as shown here:\n
    \x1b[97mlet v: Vec<i32> = Vec::new();\x1b[0m");
    println!("    the variable v contains {:?}", &v);
    println!("
    Note that we added a type annotation here. Because we aren't inserting any values into this vector,
    Rust doesn't know what kind of elements we intend to store. This is an important point. Vectors are 
    implemented using generics; we'll cover how to use generics with your own types in Chapter 10. For 
    now, know that the Vec<T> type provided by the standard library can hold any type. When we 
    create a vector to hold a specific type, we can specify the type within angle brackets. Before,
    we've told Rust that the Vec<T> in v will hold elements of the i32 type.

    More often, you'll create a Vec<T> with initial values and Rust will infer the type of value you want to 
    store, so you rarely need to do this type annotation. Rest conveniently provides the vec! macro,
    which will create a new vector that holds the values you give it. Next, we create a new Vec<i32>
    that holds the values 1, 2, and 3. The integer type is i32 because that's the default integer type,
    as we discussed in the 'Data Types' section of Chapter 3.");

    let v = vec![1, 2, 3];

    println!("
    \x1b[97mlet v = vec1[1, 2, 3];\x1b[0m 
    the variable v contains {:?}
    
    Because we've given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type 
    annotation isn't necessary. Next, we'll look at how to modify a vector.
        ", &v);
    println!("\x1b[1;4;33mUpdating a Vector\x1b[0m");
    println!("
    To create a vector and then add elements to it, we can use the push method, as shown here:");
    println!("\x1b[97m 
        let mut v: Vec<i32> = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);\x1b[0m");
    let mut v: Vec<i32> = Vec::new();
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("    the variable v contains {:?}", &v);
    println!("
    As with any variable, if we want to be able to change its value, be need to make it mutable using the 
    mut keyword, as discussed in Chapter 3. The numbers we place inside are all of type i32, and Rust 
    nifers this from the data, so we don't need the Vec<i32> annotation (I still included the annotation, 
    because I prefer to explicitly annotate the types of things.)
        ");
    println!("\x1b[1;4;33mReading Elements of Vectors\x1b[0m 

    There are two ways to reference a value stored in a vector: via indexing or by using the get method.
    In the following examples, we've annotated the types of the values that are returned from these 
    functions for extra clarity.
    Both methods for accessing a value in a vector, with indexing syntax and the get method:\x1b[97m 
        let v = vec![1, 2, 3, 4, 5];

        let third: Option<&i32> = &v[2];\x1b[0m");
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("    The third element is {third}");

    println!("\x1b[97m
        let third: Option<&i32> = v.get(2);
        match third {{
            Some(third) => println!(\"The third element is {{third}}\"),
            None => println!(\"There is no third element.\"),
        }}\x1b[0m");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("    The third element is {third}"),
        None => println!("    There is no third element."),
    }
    println!("
    Note a few details here. We use the index value of 2 to get the third element because vectors are 
    indexed by number, starting at zero. Using & and [] gives us a reference to the element at the 
    index value. When we use the get method with the index passed as an argument, we get an 
    Option<&T> that we can use with match.

    Rust provides these two ways to reference an element so you can choose how the program behaves 
    when you try to use an index value outside the range of existing elements As an example, let's see 
    what happens when we have a vector of five elements and then we try to access an element at index 
    100 with each technique, as shown here:\x1b[97m 
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);\x1b[0m 

    When we run this code, the first [] method will cause the program to panic because it references a 
    nonexistent element. This method is best used when you want your program to crash if there's an 
    attempt to access an element past the end of the vector.

    When the get method is passed an index that is outside the vector, it returns None without 
    panicking. You would use this method if accessing an element beyond the range of the vector may 
    happen occasionally under normal circumstances. Your code will then have logic to handle having 
    either Some(&element) or None, as discussed in Chapter 6. For example, the index could be coming 
    from a person entering a number. If they accedentally enter a number that's too large and the 
    program gets a None value, you could tell the user how many items are in the current vector and 
    give them another chance to enter a valid value. That would be more user-friendly than crashing the 
    program due to a typo.

    When the program has a valid reference, the borrow checker enforces the ownership and borrowing 
    rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the 
    vector remain valid. Recall the rule that states you can't have mutable and immutable references in 
    the same scope. That rule applies to the next example, where we hold an immutable reference to the first 
    element in a vector and try to add an element to the end. This program won't work if we also try to 
    try to refer to that element later in the function.\x1b[97m 
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);

        println!(\"The first element is: {{first}}\");\x1b[0m 

    Compiling this code will result in this error:\x1b[97m 
        $ cargo run
           Compiling collections v0.1.0 (file:///projects/collections)
        error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
         --> src/main.rs:6:5
          |
        4 |     let first = &v[0];
          |                  - immutable borrow occurs here
        5 |
        6 |     v.push(6);
          |     ^^^^^^^^^ mutable borrow occurs here
        7 |
        8 |     println!(\"The first element is: {{first}}\");
          |                                     ------- immutable borrow later used here

        For more information about this error, try `rustc --explain E0502`.
        error: could not compile `collections` (bin \"collections\") due to 1 previous error
\x1b[0m
    The code might look like it should work: why should a reference to the first element 
    care about changes at the end of the vector? This error is due to the way vectors work: because 
    vectors put the values next to each other in memory, adding a new element onto the end of the 
    vector might require allocating new memory and copying the old elements to the new space, if there 
    isn't enough room to put all the elements next to each other where the vector currently is stored. In 
    that case, the reference to the first element would be pointing to deallocated memory. The 
    borrowing rules prevent programs from ending up in that situation.
    ");
    
    println!("\x1b[1;4;33mIterating Over the Values in a Vector\x1b[0m 
    
    To access each element in a vector in turn, we would iterate through all of the elements rather than
    use indices to access one at a time. The next example shows how to use a for loop to get immutable 
    references to each element in a vector of i32 values and print them:\x1b[97m 
        let v = vec![100, 32, 57];

        for i in &v {{
            println!(\"{{i}}\");
        }}\x1b[0m");
    let v = vec![100, 32, 57];

    for i in &v {
        println!("    {i}");
    }
    println!("
    We can also iterate over mutable references to each element in a mutable vector in order to make 
    changes to all the elements. The next example fill add 50 to each element:\x1b[97m
        let mut v = vec![100, 32, 57];
        for i in &mut v {{
            *i += 50;
        }}\x1b[0m");
    let mut v: Vec<i32> = vec![100, 32, 57];
    println!("    v before change: {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("    v after changes: {:?}", v);
    println!("
    To change the value that the mutable reference refers to, we have to use the * dereference 
    opeator to get to the value in i before we can use the += operator. We'll talk more about the 
    dereference operator in the 'Following the Pointer to the Value with Dereference Operator'
    section of Chapter 15.

    Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules. 
    If we attempted to insert or remove items in the for loop bodies in either of these examples, we 
    would get a compiler error similar to the previous error example shown earlier. The reference to 
    the vector that the for loop holds prevents simultaneous modification of the whole vector.

\x1b[1;4;33mUsing an Enum to Store Multiple Types\x1b[0m 

    Vectors can only store values that are of the same type. This can be inconvenient; there are 
    definitely use cases for needing to store a list of items of different types. Fortunately, the variants of 
    an enum are defined under the same enum type, so when we need one type to represent elements 
    of different types, we can define and use an enum!

    For example, say we want to get values from a row in a spreadsheew in which some of the columns in 
    the row contain integers, some floating-point numbers, and some strings. We can define an enum 
    whise variants will hold the different value types, and all the enum variants will be considered the 
    same type: that of the enum. Then we can create a vector to hold that enum and so, ultimately, hold 
    different types. We've demonstrated this here:\x1b[97m 
        #[derive(Debug)]
        enum SpreadsheetCell {{
            Int(i32),
            Float(f64),
            Text(String),
        }}

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from(\"blue\")),
            SpreadsheetCell::Float(10.12),
        ]\x1b[0m");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    print!("    'row' contains: ");
    for (count, i) in (0_u8..).zip(row.iter()) {
        if count != 0 { print!(", "); }
        match i {
            SpreadsheetCell::Int(val) => print!("{val}"),
            SpreadsheetCell::Text(val) => print!("{val}"),
            SpreadsheetCell::Float(val) => print!("{val}"),
        }
    }
    println!("

    Rust need to know what types will be in the vector at compile time so it knows exactly how much
    memory on the heap will be needed to store each element. We must also be explicit about what 
    types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance 
    that one or more of the types would cause errors with the operations performed on the elements of 
    the vector. Using an enum plus a match expression means that Rust will ensure at compile time that 
    every possible case is handled, as discussed in Chapter 6.

    If you don't know the exhaustive set of types a program will get at runtime ti store in a vector, the 
    enum technique won't work. Instead, you can use a trait object, which we'll cover in Chapter 17.

    Now that we've discussed some of the most common ways to use vectors, be sure to review the API 
    documentation (https://doc.rust-lang.org/std/vec/struct.Vec.html) for all of the many useful
    methods defined on Vec<T> by the standard library. For example, in addition to push, a pop method 
    and returns the last element.

\x1b[1;4;33mDropping a Vector Drops Its Elements\x1b[0m 
    
    Like any other struct, a vector is freed when it goes out of scope, as shown here:\x1b[97m 
        {{
             let v = vec![1, 2, 3, 4];

             // do stuff with v 
        }} // <- v goes out of scope and is freed here\x1b[0m 
    
    When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will 
    be cleaned up. The borrow checker ensures that any references to contents of a vector are only 
    used while the vector itself is valid.

    The next collection type that will be covered is the String type.");
}
