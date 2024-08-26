/* Methods are similar to funcions: we declare them with the fn keyword and a name, they can
* have parameters and a return value, and they contain some code that's run when the method is
* called from somewhere else. Unlike functions, methods are defined within the context of a
* struct (or an enum or trait object), and their first parameter is always self, which
* represents the instance of the struct the method is being called on.
*/
/* Here I will change the area function from the last subchapter, and make it into a method
* defined on the Rectangle struct */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Methods can have the same names as fields, and are differentiated by the parentheses
    fn width(&self) -> bool {
        self.width > 0u32
    }
    fn can_fit(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }
    // Associated Functions:
    /* All functions defined within an impl block are called associated functions because they're
     * associated with the type named after the impl. We can define associated functions that don't
     * have self as their parameter, which wouldn't be methods, because they don't need an instance
     * of the type to work with. String::from is an example of an associated function that is not a
     * method, in this case used to construct a String.
     * These types of fuctions are often used as constructors for their structs.
     * */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // to call this type of function we use "::"; 'let sq = Rectangle::square(3);'
} // each struct is allowed to have multiple impl blocks, you are not limited to just one.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The rectangle has a nonzero width: {}", rect1.width());
    if rect1.width() {
        println!("The rectangle has a nonzero width; if is {}", rect1.width);
    } // when we use .width(), Rust knows we mean the method, and when we use .width, if knows it's
      // the field
    let rect2 = Rectangle {
        width: 20u32,
        height: 30u32,
    };
    let rect3 = Rectangle {
        width: 40u32,
        height: 20u32,
    };
    let rect4 = Rectangle {
        width: 50u32,
        height: 50u32,
    };
    println!("Can rect2 fit in rect1? {}", &rect1.can_fit(&rect2));
    println!("Can rect3 fit in rect1? {}", &rect1.can_fit(&rect3));
    println!("Can rect4 fit in rect1? {}", &rect1.can_fit(&rect4));
    let square1 = Rectangle::square(20u32);
    println!("square1 =\n{:#?}", &square1);
    println!("can rect1 fit in square1? {}", &square1.can_fit(&rect1));
}
