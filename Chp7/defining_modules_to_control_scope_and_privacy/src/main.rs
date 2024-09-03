fn main() {
    println!("Defining Modules to Control Scope and Privacy");
    println!(
        "
    In this section, we'll talk about modules and other parts of the module system, namely paths, which
    allow you to name items; the use keyword that brings a path into scope; and the pub keyword to
    make items public. We'll also discuss the as keyword, external packages, and the glob operator.
"
    );
    println!("Modules Cheat Sheet");
    println!(
        r#"
    Before we get to the details of modules and paths, here we provide a quick reference on how
    modules, paths, the use keyword, and the pub keyword work in the compiler, and how most
    developers organize their code. We'll be going through examples of each of these rules throughout
    this chapter, but this is a great place to refer to as a reminder of how modules work.
        - Start from the crate root: When compiling a crate, the compiler first looks in the crate root
          file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
        - Declaring modules: In the crate root file, you can declare new modules; say you declare a
          "garden" module with `mod garden;`. The compiler will look for the module's code in these
          places:
            - Inline, within curly brackets that replace the semicolon following `mod garden`
            - In the file src/garden.rs
            - In the file src/garden/mod.rs
        - Declaring submodules: In any file other than the crate root, you can declare submodules. For
          example, you might declare `mod vegetables;` in src/garden.rs. The compiler will look for the
          submodule's code within the directory named for the parent module in these places:
            - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
            - In the file src/garden/vegetables.rs
            - In the file src/garden/vegetables/mod/rs
        - Paths to code in modules: Once a module is part of your crate, you can refer te code in that
          module from anywhere else in that same crate, as long as the privacy rules allow, using the
          path to the code. For example, an Asparagus type in the garden vegetables module would be
          found at crate::garden::vegetables::Asparagus.
        - Private vs. public: Code within a module is private from its parent modules by default. To
          make a module public as well, use pub before their declarations.
        - The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce
          repetition of long paths. In any scope that can refer to
          crate::garden::vegetables::Asparagus, you can create a shortcut with:
          use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus
          to make use of that type in the scope.
"#
    );
}
