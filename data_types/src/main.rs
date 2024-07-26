fn main() {
    // use 'let' keyword to assign a value to an identifier.
    let string = "string";
    println!("this is a string: {string}");
    // once a value is assigned to an identifier, it cannot be changed unless you re-assign the
    // identifier with another 'let'. This is called shadowing.
    let string = "newstring";
    println!("this is a new string: {string}");

    // if you want a variable to be mutable, then you have to add the 'mut' keyword after the let.
    let mut mutable_string = "this is a mutable string";
    println!("{mutable_string}");
    mutable_string = "the mutable string has been changed";
    println!("{mutable_string}");
    // once a mutable variable has been assigned a type, it can only be changed to values of that
    // same type.

    /* Scalar Types */
    // A scalar type represents a single value. Rust has four primary scalar types: Integers,
    // floating-point numbers, Booleans, and Characters

    /* Integer Types */
    // An Iteger is a number without a fractional component. There can be 8-bit, 16-bit, 32-bit,
    // 64-bit, 128-bit and arch ints (32 or 64 bit depending on system architecture)

    // each bit level can be signed or unsigned, such as u8 for an unsigned 8-bit integer, or i64
    // for a signed 64-bit integer. 'signed' means an integer can be negative or positive and while
    // be prefixed with a + or -, while 'unsigned' means that the integer can only be positive and will
    // not have a prefix because its sign is implicit.

    /* examples for all built-in integer types */
    let eight_bit_u: u8 = 1;
    let eight_bit_i: i8 = 2;
    let sixteen_bit_u: u16 = 3;
    let sixteen_bit_i: i16 = -4;
    let thirtytwo_bit_u: u32 = 5;
    let thirtytwo_bit_i: i32 = 6;
    let sixtyfour_bit_u: u64 = 7;
    let sixtyfour_bit_i: i64 = 8;
    let onetwentyeight_bit_u: u128 = 9;
    let onetwentyeight_bit_i: i128 = 10;
    let arch_u: usize = 11;
    let arch_i: isize = 12;

    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        eight_bit_u,
        eight_bit_i,
        sixteen_bit_u,
        sixteen_bit_i,
        thirtytwo_bit_u,
        thirtytwo_bit_i,
        sixtyfour_bit_u,
        sixtyfour_bit_i,
        onetwentyeight_bit_u,
        onetwentyeight_bit_i,
        arch_u,
        arch_i
    );

    // you can assign an integer variable without type annotation, and the rust compiler will most
    // likely default to i32, which could lead to errors if the variable is assigned a value that
    // exceeds those bounds. It's best practice to always annotate the data type.

    /* Floating Point Numbers */
    // floats of different types cannot be added
    let floattt: f32 = 2.5;
    let floattt_2: f32 = 1.0;
    let floatsf: f64 = 6.5;
    let floatsf_2: f64 = 2.0;

    println!(
        "Adding floats: {} {}",
        floattt + floattt_2,
        floatsf + floatsf_2
    );

    /* Operations */
    println!(
        "{} {} {} {} {} {}",
        5 + 5,
        95.5 - 2.24,
        4 * 30,
        -15 / 3,
        5 / 3,
        5 % 3
    );
    println!("{}", 95.2 / 2.5);
    println!("{}", 10 / 3);
    println!("{}", 10 / 2);
    // if you want exact division, you need to use floats, otherwise it will truncate everything
    // after the decimal
    println!("{}", 10.0 / 3.0);

    /* Booleans */
    // Booleans can be either true or false, and are one byte in length.
    let blean: bool = false;
    println!("{blean}");

    /* Character Type */
    // rust's char type is the language's most primitive alphabetic type. Here are some examples of
    // declaring char values:
    let y = 'c';
    // char uses unicode values, which means it can contain special symbols and emojis
    let z: char = 'ℤ';
    let emoji: char = '😂';

    // chars must be contained in single quotes, as opposed to string literals, which are contained
    // in double quotes. Rust's char type is four bytes in size.
    println!("{z} {y} {emoji}");

    /* Compound Types */
    /* Tuples */
    // a tuple is a general way of grouping together a number of values with a variety of types
    // into one compound type. Tuples have a fixed length; once declared, they cannot grow or
    // shrink in size.

    // Creating a tuple:
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // There are two ways we can access tuples.
    // Through unpacking (This is the more idiomatic and preferred way of accessing elements)
    let (a, b, c) = tup;
    println!("the tuple values are: {a}, {b}, {c}");

    // and directly through the index:
    println!("the tuple values are: {0}, {1}, {2}", tup.0, tup.1, tup.2);

    // A tuple without any values is called a 'unit'. This value and it's type are both written
    // '()' and represent an empty value or an empty return type. Expressions implicitly return the
    // unit value if they don't return any other value.

    /* Arrays */
    // Another way to have a collection of mutliple values is with an array. Unlike a tuple, every
    // Arrays have a fixed length, and its elements must all be of the same type.

    // To annotate time for a list, use square brackets containing the type, a semicolon, and the
    // number of elements.
    let myarray: [i32; 5] = [1, 2, 3, 4, 5];
    println!("this is the first element in myarray: {0}", myarray[0]);
    for num in myarray {
        println!("{num}")
    }

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let june = months[5];
    println!("This is the 6th month of the year: {june}");
    for month in months {
        println!("{month}");
    }

    // You can also initialize an array to contain the same value for each element by specifying
    // the initial value, followed by a semicolon, and then the length of the array in square
    // brackets.
    let threesarray: [u8; 5] = [3; 5];
    for val in threesarray {
        println!("{val}");
    }
    // If an array is directly accessed through user input, it's reccomended to validate the input
    // before trying to access the array index, as an out of bounds index will panic the program.
    // Note: a similar data type is a vector, which is provided by the standard library. It can
    // grow or shrink in size.

    println!("These are all of Rust's build-in data types!");
}
