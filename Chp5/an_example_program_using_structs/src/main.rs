fn main() {
    let width1: u32 = 30;
    let hight1: u32 = 50;

    println!("The area of the rectangle is {}px.", area(width1, hight1));

    let rect1: (u32, u32) = (30u32, 50u32);
    println!(
        "The area of the rectangle (with tuple param) is {}px.",
        tup_area(rect1)
    );
    mini_main();
}
/* To understand why we might want to use structs, let's write a program that calculates the area
* of a rectangle. We'll use single variables first, then refactor the function until we're using
* structs instead. */
fn area(width: u32, height: u32) -> u32 {
    width * height
}
/* This code succeeds in figuring out the area of the rectangle by calling the area function with
* each dimension, but we can do more to make this code clear and readable.
* The issue with the code is in the variables for area. There is no indication in the program that
* the parameters for width and height are related. It would be more manageable to group width and
* height together. */
// Here is a version that uses tuples:
fn tup_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
/* While this function is better, it's still less clear. tuples don't name their elements, so we
* have to index into the parts of the tuple, making our calculation less obvious.
* Mixing up the width and height wouldn't matter for the area calculation, but if we want to draw
* the rectangle on the screen, for example, it would matter. We would have to keep in mind which
* index matches with width and which matches with height. Because we haven't conveyed the meaning
* of the data in our code, it's now easier to introduce errors. */
// Now using structs:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn mini_main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle (with a struct) is {}px.",
        struc_area(&rect1)
    );
    println!("rect1 is {rect1:?}"); // you can't print a struct normally.
                                    // You have to use '{:?}', and the opt-in the struct with the Debug trait to print out the
                                    // contents of the struct. '{:#?}' can also be used to pretty print the contents
    dbg!(&rect1); // The dbg!() macro can also be used for this purpose

    let scale: u32 = 2u32;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // dbg! can also display an expression, and it will return
        // ownership of the expression's value so that the code functions the same way it would if
        // dbg! wasn't there.
        height: 50,
    };
    dbg!(&rect2); // a reference is used because we don't want dbg! to take ownership here
}
fn struc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
/* Now how the data is grouped and what each value means is much more clear. */
// Debug is one of many useful traits Rust has provided us to use with the derive attrubute, which
// can add behaviour to custom types.
