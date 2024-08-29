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
}
