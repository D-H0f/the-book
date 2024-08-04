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
}
