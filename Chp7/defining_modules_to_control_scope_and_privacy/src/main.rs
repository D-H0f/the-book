fn main() {
    println!("\x1b[1;4;33mDefining Modules to Control Scope and Privacy\x1b[0m");
    println!(
        "
    In this section, we'll talk about modules and other parts of the module system, namely paths, which
    allow you to name items; the use keyword that brings a path into scope; and the pub keyword to
    make items public. We'll also discuss the as keyword, external packages, and the glob operator.
"
    );
    println!("\x1b[1;4;33mModules Cheat Sheet\x1b[0m");
    println!(
        "
    Before we get to the details of modules and paths, here we provide a quick reference on how
    modules, paths, the use keyword, and the pub keyword work in the compiler, and how most
    developers organize their code. We'll be going through examples of each of these rules throughout
    this chapter, but this is a great place to refer to as a reminder of how modules work.
        - \x1b[1;92mStart from the crate root\x1b[0m: When compiling a crate, the compiler first looks in the crate root
          file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
        - \x1b[1;92mDeclaring modules\x1b[0m: In the crate root file, you can declare new modules; say you declare a
          \"garden\" module with `mod garden;`. The compiler will look for the module's code in these
          places:
            - Inline, within curly brackets that replace the semicolon following `mod garden`
            - In the file src/garden.rs
            - In the file src/garden/mod.rs
        - \x1b[1;92mDeclaring submodules\x1b[0m: In any file other than the crate root, you can declare submodules. For
          example, you might declare `mod vegetables;` in src/garden.rs. The compiler will look for the
          submodule's code within the directory named for the parent module in these places:
            - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
            - In the file src/garden/vegetables.rs
            - In the file src/garden/vegetables/mod/rs
        - \x1b[1;92mPaths to code in modules\x1b[0m: Once a module is part of your crate, you can refer te code in that
          module from anywhere else in that same crate, as long as the privacy rules allow, using the
          path to the code. For example, an Asparagus type in the garden vegetables module would be
          found at crate::garden::vegetables::Asparagus.
        - \x1b[1;92mPrivate vs. public\x1b[0m: Code within a module is private from its parent modules by default. To
          make a module public as well, use pub before their declarations.
        - \x1b[1;92mThe use keyword\x1b[0m: Within a scope, the use keyword creates shortcuts to items to reduce
          repetition of long paths. In any scope that can refer to
          crate::garden::vegetables::Asparagus, you can create a shortcut with:
          use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus
          to make use of that type in the scope.

    Here, we create a binary crate named backyard that illustrates these rules. The crate's directory,
    also named backyard, contains these files and directories:\x1b[97m
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs\x1b[0m
    The crate root file in this case is \x1b[3msrc/garden.rs\x1b[0m, and it contains:\x1b[97m
    use crate::garden::vegetables::Asparagus;

    pub mod garden;

    fn main() {{
        let plant = Asparagus {{}};
        println!(\"I'm growing {{plant:?}}\");
    }}\x1b[0m
    The pub mod garden; line tells the compiler to include the code it finds in \x1b[3msrc/garden.rs\x1b[0m, which is:\x1b[97m
    pub mod vegetables;\x1b[0m
    Here, pub mod vegetables; means the code in src/garden/vegetables.rs is included too. That code is:\x1b[97m
    #[derive(Debug)]
    pub struct Asparagus {{}}\x1b[0m
    Now let's get into the details of these rules and demonstrate them in action!

\x1b[1;4;33mGrouping Related Code in Modules\x1b[0m

    \x1b[3mModules\x1b[0m let us organize code within a crate for readability and easy reuse. Modules also allow us to
    control the \x1b[3mprivacy\x1b[0m of items because code within a module is private by default. Private items are
    internal implementation details not available for outside use. We can choose to make modules and
    the items within them public, which exposes them to allow external code to use and depend on
    them.

    As an example, let's write a library crate that provides the functionality of a restaurant. We'll define
    the signatures of functions but leave their bodies empty to concentrate on the organization of the
    code rather than the implementation of a restaurant.

    In the restaurant industry, some parts of a restaurant are referred to as \x1b[3mfront of house\x1b[0m and others as
    \x1b[3mback of house\x1b[0m. Front of house is where customers are; this encompasses where the hosts seat
    customers, servers take orders and payment, and bartenders make drinks. Back of house is where
    the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative
    work.

    To structure our crate in this way, we can organize its functions into nusted modules. Create a new
    library named \x1b[3mrestaurant\x1b[0m by running \x1b[97mcargo new restaurant --lib\x1b[0m. Then enter the code in the
    following example into \x1b[3msrc/lib.rs\x1b[0m to define some modules and function segnatures; this code is the front of house
    section.\x1b[97m
mod front_of_house {{
    mod hosting {{
        fn add_to_waitlist() {{}}

        fn seat_at_table() {{}}
    }}

    mod serving {{
        fn take_order() {{}}

        fn serve_order() {{}}

        fn take_payment() {{}}
    }}
}}\x1b[0m
    We define a module with the mod keyword followed by the name of the module (in this case,
    front_of_house). The body of the module then goes inside curly brackets. Inside modules, we can
    place other modules, as in this case with the modules hosting and serving. Modules can also hold
    definitions for other items, such as structs, enums, constants, traits, and functions.

    By using modules, we can group related definitions together and name why they're related.
    Programmers using this code can navigate the code based on the groups rather than having to read
    through all the definitions, making it easier to find the definitions relevant to them. Programmers
    adding new functionality to this code would know where to place the code to keep the program
    organized.

    Earlier, we mentioned that \x1b[3msrc/main.rs\x1b[0m and \x1b[3msrc/lib.rs\x1b[0m are called roots. The reason for their name
    is that the contents of either of these two files form a module crate at the root of the
    crate's module structure, known as the module tree.

    The following example shows the module tree for the structure in the previouse code example:\x1b[97m
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment\x1b[0m

    This tree shows how some of the modules nest inside other modules; for example, hosting nests
    inside front_of_house. The tree also shows that some modules are siblings, meaning they're
    defined in the same module; hosting and serving are siblings defined within front_of_house. If
    module A is contained inside module B, we say that module A is the child of module B and that
    module B is the parent of module A. Notice that the entire module tree is rooted under the implicit
    module named crate.

    The module tree might remind you of the filesystem's directory tree on you computer; this is a very
    apt comparison. Just like directories in a filesystem, you use modules to organize your code. And just
    like files in a directory, we need a way to find our modules.
"
    );
}
