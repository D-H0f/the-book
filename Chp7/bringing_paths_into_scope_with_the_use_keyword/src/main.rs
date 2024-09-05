fn main() {
    println!("\x1b[1;4;33mBringing Paths into Scope with the use Keyword\x1b[0m 

    having to write out the paths to call functions can feel inconvenient and repetative. In the last subchapter,
    whether we chose the absolute or relative path to the add_to_waitlist function, every time we 
    wanted to call add_to_waitlist we had to specify front_of_house and hosting too. Fortunately,
    there's a way to simplify this process: we can create a shortcut to a path with the use keyword once,
    and then use the shorter name everywhere else in the scope.

    In the next example, we bring the crate::front_of_house::hosting module into the scope of the 
    eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the 
    add_to_waitlist function in eat_at_restaurant:\x1b[97m]
mod front_of_house {{
    pub mod hosting {{
        pub fn add_to_waitlist() {{}}
    }}
}}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {{
    hosting::add_to_waitlist();
}}\x1b[0m 
    
    Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding 
    use \x1b[97mcrate::front_of_house::hosting\x1b[0m in the crate root, hosting is now a valid name in that scope,
    just as though the hosting module had been defined in the crate root. Paths brought into scope 
    with use also check privacy, like any other paths.

    Note that use only creates the shortcut for the particular scope in which the use occurs. The following example
    moves the eat_at_restaurant functoin into a new child module named customer, which is then
    a different scope than the use statement, so the function won't compile.\x1b[97m 
mod front_of_house {{
    pub mod hosting {{
        pub fn add_to_waitlist() {{}}
    }}
}}

use crate::front_of_house::hosting;

mod customer {{
    pub fn eat_at_restaurant() {{
        hosting::add_to_waitlist();
    }}
}}\x1b[0m

    The compiler error shows that the shortcut no longur applies within the customer module:\x1b[97m 
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
\x1b[0m

    Notice there's also a warning that the use is no longer used in its scope! To fix this problem, move 
    the within the customer module too, or reference the shortcut in the parent module with 
    super::hosting within the child customer module.

\x1b[1;4;33mCreating Idiomatic use Paths\x1b[0m 

    In the first example, you might have wondered why we specified ure crate::front_of_house::hosting 
    and then called hosting::add_to_waitlist in eat_at_restaurant, rather than specifying the use 
    path all th way out to the add_to_waitlist function to achieve the same result, as in the following example:
\x1b[97m\
mod front_of_house {{
    pub mod hosting {{
        pub fn add_to_waitlist() {{}}
    }}
}}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {{
    add_to_waitlist();
}}\x1b[0m

    Although both these examples accomplish the same task, the former is the idiomatic
    way to bring a function into scope with use. Bringing the function's parent module into scope with 
    use means we have to specify the parent module when calling the function. Specifying the parent 
    module when calling the function makes it clear that the function isn't locally defined while still
    minimizing repetition of the full path. The code in the previous example is unclear as to where 
    add_to_waitlist is defined.

    On the other hand, when bringing in structs, enums, and other items with use, it's idiomatic to 
    specify the full path. The next example shows the idiomatic way to bring the standard library's HashMap 
    struct into the scope of a binary crate:\x1b[97m 
use std::collections::HashMap;

fn main() {{
    let mut map = HashMap::new();
    map.insert(1, 2);
}}\x1b[0m

    There's no strong reason behind this idiom: it's just the convention that has emerged, and people have 
    gotten used to reading and writing Rust code this way.

    The exception to this idiom is if we're bringing two items with the same name into scope with use 
    statements, because Rust doesn't allow that. The next example shows how to bring two Result types into 
    scope that have the same name but different parent modules, and how to refer to them:\x1b[97m 
use std::fmt;
use std::io;

fn function1() -> fmt::Result {{
    // --snip--
}}

fn function2() -> io::Result<()> {{
    // --snip--
}}\x1b[0m

    As you can see, using the parent modules distinguishes the two Result types. If instead we 
    specified \x1b[97muse std::fmt::Result\x1b[0m and \x1b[97muse \
std::io::Result\x1b[0m, we'd have two Result types in the 
    same scope, and Rust wouldn't know which one we meant when we used Result.

\x1b[1;4;33mProviding New names with the as Keyword\x1b[0m

    There's another solution to the problem of bringing two types of the same name into the same 
    scope with use: after the path, we can specify as and a new local name, or alias, for the type.
    The next example shows another way to write the code from the previous by renaming one of the two Result 
    types using as:\x1b[97m 
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {{
    // --snip--
}}

fn function2() -> IoResult<()> {{
    // --snip--
}}\x1b[0m

    In the second use statement, we chose the new name IoResult for the std::io::Result type,
    which won't conflict with the Result from std::fmt that we've also brought into scope. Both 
    of these examples are considered idiomatic, so the choice is up to you.

\x1b[1;4;33mRe-exporting Names with pub use\x1b[0m 

    When we bring a name into scope with the use keyword, the name available in the new scope is 
    private. To enable the code that calls our code to refer to that name as if it had been defined in that 
    code's scope, we can combine pub and use. This technique is called re-exporting because we're 
    bringing an item into scope but also making that item available for others to bring into their scope.

    This next example shows the code in the first example with use in the root module changed to 'pub use':\x1b[97m 
mod front_of_house {{
    pub mod hosting {{
        pub fn add_to_waitlist() {{}}
    }}
}}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {{
    hosting::add_to_waitlist();
}}\x1b[0m

    Before this change, external code would have to call the add_to_waitlist function by using the 
    path restaurant::front_of_house::hosting::add_to_waitlist(), which also would have required 
    the front_of_house module to be marked as pub. Now that this pub use has re-exported the 
    hosting module from the root module, external code can use the path restaurant::hosting::add_to_waitlist()
    instead.

    Re-exporting is useful when the internal structure of your code is differeng from how programmers 
    calling your code would think about the domain. For example, in this restaurant metaphor, the 
    people running the restaurant think about 'front of house' and 'back of house'. But customers
    visiting a restaurant probably won't think about the parts of the restaurant in those terms. With pub 
    use, we can write our code with one structure but expose a different structur. Doing so makes our 
    library well organized for programmers working on the library and programmers calling the library.
    We'll look at another example of pub use and how it affects your crate's decumentation in the
    'Exporting a Convenient Public API with pub use' section of Chapter 14.

\x1b[1;4;33mUsing External Packages\x1b[0m 
    
    In Chapter 2, we programed a guessing game project that used an external package called rand
    to get random numbers. To use rand in our project, we added this line to Cargo.toml:\x1b[97m 
rand = \"0.8.5\"\x1b[0m 
    
    Adding rand as a dependency in cargo.toml tells Cargo to download the rand package and any 
    dependencies from crates.io and make rand available to our project. 

    Then, to bring rand definitions into the scope of our package, we added a use line starting with the 
    name of the crate, rand, and listed the items we wanted to bring into scope. Recall that in the 
    'Generating a Random Number' section in Chapter 2, we brought the Rng trait into scope and called 
    the rand::thread_rng functoin:\x1b[97m 
use rand::Rng;

fn main() {{
    let secret_number = rand::thread_rng().gen_range(1..=100);
}}\x1b[0m

    Members of the rust community have made many packages available at crates.io, and pulling any of 
    them into your package involves these same steps: listing them in your package's Cargo.toml file and 
    using use to bring items from their crates into scope. 

    Note that the standard std library is also a crate that's external to our package. Because the 
    standard library is shipped with the Rust language, we don't need to change Cargo.toml to include 
    std. But we do need to refer to it with use to bring items from there into our package's scope. For 
    example, with HashMap we use this line:\x1b[97m 
use std::collections::Hasmap;\x1b[0 
    This is an absolute path starting with std, the name of the standard library crate.

\x1b[1;4;33mUsing Nested Paths to Clean Up Large use Lists\x1b[0m

    If we're using multiple items defined in the same crate or same module, listing each item on its own 
    line can take up a lot of vertical space in out files. For example, these two use statements we had in 
    the guessing game bring items from std into scope:\x1b[97m 
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--\x1b[0m
    Instead, we can use nested paths to bring the same items into scope in one line. We do this by 
    specifying the common part of the path, followed by two colons, and then curly brackets around a 
    list of the parts of the paths that differ, as shown here:\x1b[97m 
use std::{{cmp::Ordering, io}};\x1b[0m
    In bigger programs, bringing many items into scope from the same crate or module using nested 
    paths can reduce the number of separate use statements needed by a lot.

    We can use a nested path at any level in a path, which is useful when combining two use 
    statements that share a subpath. For example, the next example shows two use statements: one that 
    brings std::io into scope and one that brings std::io::Write into scope:\x1b[97m 
use std::io;
use std::io::Write;\x1b[0m 
    The common part of these two paths is std::io, and that's the complete first path. To merge these 
    two paths into one use statement, we can use self in the nested path, as shown here:\x1b[97m 
use std::io::{{self, Write}};\x1b[0m 
    This line brings std::io and std::io::Write into scope.

\x1b[1;4;33mThe Glob Operator\x1b[0m 
    
    If we want to bring all public items defined in a path into scope, we can specify that path followed by 
    the '*' glob operator:\x1b[97m 
use std::collections::*;\x1b[0m 
    This use statement brings all public items defined in std::collections into the current scope. Be 
    careful when using the glob operator. Glob can make it harder to tell what names are in scope and 
    where a name used in your progam was defined.

    The glob operator is often used when testing to bring everything under test into the tests module; 
    we'll talk about that in the 'How te Write Tests' section in Chapter 11. The glob operator is also 
    sometimes used as part of the prelude pattern: see the standard library documentation
    (https://doc.rust-lang.org/std/prelude/index.html#other-preludes)
    for more information on that pattern.
");
}
