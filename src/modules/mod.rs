pub fn explain() {
    println!("Here's a quick reference for modules:");
    /*
        1. Start from crate root
            - When a crate is compiled, first compiler looks in crate root (prob src/main.rs or src/lib.rs)
        2. Module declaration
            - You can only declare a new module in the crate root 
            - Look at the top of main.rs for an example                                                                
            - The code to populate the module can be found in the following places:
                - In curly braces after the mod name instead of a normal semicolon
                - In src/mod-name.rs
                - In src/mod-name/mod.rs
        3. Submodules
            - In any file that isn't the crate root, submodules can be created.
            - For example, this mod is a submod of the mod modules.rs 
            */
            mod submodules { pub fn say_hi() {println!("hey, i'm a submodule!")} }
            /*
            - This submodule, like a normal mod, can be defined in curly braces, as well as:
                - In src/mod_name/submod_name.rs
                - In src/mod_name/submod_name/mod.rs
        4. Accessing code in modules
            - Once you make a mod part of your code with mod, 
                you can refer to it from anywhere else in the crate if privacy rules allow (see 5)
            */
            crate::mod_name::hi();
            crate::mod_name::submod_name::hi();
            /*
        5. Privacy rules (private and public)
            - Code in mods is private by default: to make code accessible to other modules, declare as pub mod
            - To make specific functions and variables public, declare pub fn or pub let
            */
            for kanji in crate::mod_name::THE_FIVE_NEW_KANJI {
                println!("{}", kanji);
            }
            /*
        6. The use keyword
            - If you declare a path with use instead of mod, then you can refer to that path quickly
            */
            use crate::mod_name::GLOBS_ARE_BAD;
            println!("{}", &GLOBS_ARE_BAD[16..]);
            /*
    */

    println!("Okay, now let's go over modules in detail!");
}


//Modules group code for readibility and easy reuse.
//They allow code to be obscured from other files in a crate (code in a crate is private by default)
//For example, let's make a library crate that represents a restaurant.
//Look for a restaurant crate!