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
    fn can_fit(&self, &other: Rectangle) -> bool {
        {
            self.width > other.width && self.length > other.length
        }
        || self.width > other.length && self.length > other.width
    }
}
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
}
