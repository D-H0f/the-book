/* Structs are similar to tuples in that both old multiple related values. Like tuples, the pieces
* of a struct can be different types. Unlike tuples, in a struct you'll name each piece of data so
* it's clear what the values mean. Adding these names means that structs are more flexable than
* tuples; you don't hve to rely on the order of the data to specify or access the values of an
* instance */
struct User {
    // The 'struct' keyword is used to define a struct, like class in python
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // To use a defined struct, we create an instance with 'let' and specify values for it's
    // attributes.
    let user1 = User {
        active: true,
        username: String::from("fooman"),
        email: String::from("fooman@gmail.com"),
        sign_in_count: 1u64,
    };
    // to access, use dot notation with the instance identifier:
    println!(
        "user1: {}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // To change attributes, the entire instance must be mutable. Rust does not allow us to make
    // only specific fields mutable:
    let mut mutauser = User {
        active: false,
        username: String::from("barman"),
        email: String::from("barman@gmail.com"),
        sign_in_count: 2u64,
    };
    println!(
        "mutauser: {}, {}, {}, {}",
        mutauser.active, mutauser.username, mutauser.email, mutauser.sign_in_count
    );
    (mutauser.active, mutauser.username) = (true, String::from("anothername"));
    println!(
        "mutauser: {}, {}, {}, {}",
        mutauser.active, mutauser.username, mutauser.email, mutauser.sign_in_count
    );
    let newuser = build_user(
        String::from("sockpuppet@fun.com"),
        String::from("sockpuppet"),
    );
    println!(
        "newuser: {}, {}, {}, {}",
        newuser.active, newuser.username, newuser.email, newuser.sign_in_count,
    );
    // Here's the naive way to create a new struct instance that includes field values from another
    // struct:
    /*
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@gmail.com"),
        sign_in_count: user1.sign_in_count
    }; */
    // here is the better way, using the built-in update syntax:
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // This syntax allows us to only assign the field values that we want to change, and '..user1'
    // will fill in any that have not been explicitly assigned.
    println!("{} {}", user1.email, user2.email);
    using_tuple_structs_without_named_fields_to_create_different_types();
}
// To set default values for fields, we can use a function that constructs the struct:
fn build_user(email: String, username: String) -> User {
    // because the function parameters and the struct field names are exactly the same, you can use
    // the field init shorthand so you dont have to write 'username = username' or some equivilant
    // many times:
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn using_tuple_structs_without_named_fields_to_create_different_types() {
    /* another type of struct supported by Rust are tuple structs, named for their similarity to
     * tuples. Tuple structs have the added meaning the struct name provides, but don't have names
     * associated with their fields; rather, they just have the types of the fields*/
    // here is how tuple structs are defined:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0i32, 0i32, 0i32);
    let origin = Point(0i32, 0i32, 0i32);
    println!("black: {} {} {}", black.0, black.1, black.2);
    println!("origin: {} {} {}", origin.0, origin.1, origin.2);
}
fn _unit_like_structs_without_any_fields() {
    /* You can also define structs that don't have any fields. These are called unit-like structs,
     * because they behave similarly to (), the unit type. Unit-like structs can be useful when you
     * need to implement a trait on some type but don't have any data that you want to store in the
     * type itself. This will be expanded upon in chapter 10. */
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}
/* In the User struct definition made earlier, we used the owned String type rather than the &str
* string slice type. This is a deliberate choice because we want each instance of this struct to
* own all of it's data and for that data to be valid for as long as the entire struct is valid. */

/* It's also possible for structs to store reference to data owned by something else, but to do so
* requires the use of lifetimes, a Rust feature that will be discussed in Chapter 10. Lifetimes
* ensure that the data referenced by a struct is valid for as long as the struct is. */
// Trying to store a reference in a struct without specifying a lifetime, like this, wont work:
/*
struct NewUser {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u8,
}
fn ref_without_lifetime() {
    let user2 = NewUser {
        active: true,
        username: "someuser",
        email: "someuser@gmail.com",
        sign_in_count: 1,
    };
// This will fail to compile, with the error specifying that lifetime parameters are required.
}*/
