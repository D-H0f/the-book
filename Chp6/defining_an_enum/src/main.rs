// Defining An Enum
/* Where structs give you a way of grouping together related fields and data, like a Rectangle with
* its width and height, enums give you a way of saying a value is one of a possible set of values.
* For example, we may wand to say that Rectangle is one of a set of possible shapes that also
* includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum
* Let's look at a situation we might want to express in code and see why enums are useful and more
* appropriate than structs in this case. Say we need to work with IP addresses. Currently, two
* major standards are used for IP addresses: version four and version six. Because these are the
* only possibilities for an IP address that our program will come across, we can enumerate all
* possible variants, which is where enumeration gets its name.
* Any IP address can be either a version four or a version six address, but not both at the same
* time. That property of IP addresses makes the enum data structure appropriate because an enum
* value can only be one of its variants. Both versions are still fundamentally IP addresses, so
* they should be treated as the same type when the code is handling situatoins that apply to any
* kind of IP address.
* We can express this concept in code by defining an IpAddrKind enumeration and listing the
* possible kinds an IP address can be, V4 and V6. These are the variants of this enum.
* */
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
} // IpAddrKind is now a custom data type that we can use in our code
  /* We can then call this function with either variant */
fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
/* Here is how we can bundle data directly in each enum variant right in the enum block: */
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
/* Here is an example of an enum with a wide variety of types embedded in its variants:
* */
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Enum Methods
/* There is one more similarity between enums and structs: just as we're able to define methods on
* structs using impl, we're also able to define methods on enums. Here's a method named call that
* we could define on our Message enum:
* */
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quitting..."),
            Message::Move { x, y } => println!("Moving to position {}, {}", x, y),
            Message::Write(message) => {
                println!("Writing message \"{}\" to current positon", message)
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to the value ({}, {}, {})", r, g, b)
            }
        }
    }
}
// The option Enum and its Advantages over Null Values
/* Next we will be going over Option, an enum that is defined in the standard library. The Option
* type encodes the very common scenario in which a value could be something or it could be nothing.
* For example, if you request the first item in a non-empty list, you would get a value. If you
* request the first item in an empty list, you would get nothing. Expressingt this concept in terms
* of the type system means the compiler can check whether you've handled all the cases you should
* be handling; this functionality can prevent bugs that are emtremely common in other programming
* languages.
* Programming language design is often thought of in terms of which features you include, but the
* features you exclude are important too. Rust doesn't have the null feature that many other
* languages have. Null is a value that means there is no value there. In languages with null,
* variables can always be in one of two states; null or not-null.
* The problem with null values is that if you try to use a null value as a not-null value, you'll
* get an error of some kind. Because this null or not-null property is pervasive, it's extremely
* easy to make this kind of error.
* However, the concept that null is trying to express is still a usefell one: a null is a value
* that is currently invalid or absent for some reason.
* The problem isn't really with the concept but with the particular implementation. As such, Rust
* does not have nulls, but it does have an enum that can encode the concept of a value being
* present or absent. This enum is Option<T>, and it is defined by the standard library as follows:
* enum Option<T> {
* None,
* Some(T),
* }
* The Option<T> enum is so useful that it's even included in the prelude; you don't need to bring
* it into scope explicitly. It's variants are also included in the prelude: you can use Some and
* None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and
* Some(T) and None are still variants of type Option<T>.
* The <T> sytax is a feature of Rust we haven't talked about yet. It's a generic type parameter,
* and we'll cover generics in more detail in Chapter 10. For now, all you need to know is that <T>
* means that the Some variant of the Option enum can hold one piece of data of any type, and that
* each concrete type that gets used in place of T makes the overall Option<T> type a different
* type. Here are some examples of using Option values to hold number types and string types:
* */
fn option_enum_examples() {
    let some_number = Some(5u8);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    if let Some(num) = &some_number {
        println!("{}", num);
    }
    if let Some(c) = some_char {
        println!("{}", c)
    }
    match &absent_number {
        Some(num) => println!("{}", num),
        None => println!("The variable is not currently holding a value."),
    }
    /* The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a
    * different type. Rust can infer these types because we've specified a value inside the Some
    * variant. For absent_number, Rust requires us to annotate the overall Option type: the
    * compiler can't infer the type that the corresponding Some variant will hold by looking only
    * at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    * When we have a Some value, we know that a value is present and the value is help within the
    * Some. When we have a None value, in some sense it means the same thing as null: we don't have
    * a valid value. So why is having Option<T> any better than having null?
    * In short, because Option<T> and T (where T can be any type) are different types, the compiler
    * won't let us use an Option<T> value as if it were definitely a valid value. For example, this
    * code won't compile, because it's trying to add an i8 to an Option<i8>:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    * The compiler will throw an error because it does not unserstand how to add an i8 and an
    * Option<i8>, because they're different types. When we have a value of a type i8 in Rust, the
    * compiler will ensure that we always have a valid value. Only when we have an Option<i8> (or
    * whatever type of value we're working with) do we have to worry about possibly not having a
    * value, and the compiler will make sure we handle that case before using the value.
    * In other words, you have to convert an Option<T> to a T before you can perform T operations
    * with it. Genrally, this helps catch one of the most common issues with null: assuming that
    * something isn't null when it actually is.
    * */
}
fn main() {
    // Enum Values
    /* we can create instances of each of the two variants like this: */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?} {:?}", four, six);
    route(IpAddrKind::V4);
    route(six);

    // Ip address variant enums bundled with their data:
    let home = IpAddr::V4(127u8, 0u8, 0u8, 1u8);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?} {:?}", home, loopback);
    println!("{:#?}", home);
    // Here are two ways to access the values of an enum:
    match &home {
        IpAddr::V4(first, second, third, fourth) => {
            println!("First octet: {}", first);
            println!("Second octet: {}", second);
            println!("Third octet: {}", third);
            println!("Fourth octet: {}", fourth);
        }
        IpAddr::V6(s) => println!("IPv6 address: {}", s),
    }
    // and the other...
    if let IpAddr::V4(first, second, third, fourth) = &home {
        println!("First octet: {}", first);
        println!("Second octet: {}", second);
        println!("Third octet: {}", third);
        println!("Fourth octet: {}", fourth);
    }
    let quit_message = Message::Quit;
    println!("{:?}", quit_message);
    let move_message = Message::Move { x: 5, y: 10 };
    let write_message = Message::Write(String::from("this is a message"));
    let color_change = Message::ChangeColor(30i32, 20i32, 80i32);
    println!("{:#?}", move_message);
    println!("{:#?}", color_change);
    println!("{:#?}", write_message);
    if let Message::Move { x, y } = &move_message {
        println!("x, y : {}, {}", x, y);
    }
    if let Message::Write(message) = &write_message {
        println!("{}", message);
    }
    if let Message::ChangeColor(r, g, b) = &color_change {
        println!("new color value is: {},{},{}", r, g, b);
    }
    quit_message.call();
    move_message.call();
    write_message.call();
    color_change.call();
    color_change.call();
    option_enum_examples();
}
