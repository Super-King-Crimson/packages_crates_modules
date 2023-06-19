//If you're looking for a restaurant, this isn't the right one. I'm being serious. Keep looking.
#![allow(unused, dead_code)]

//front_of_house is its own file, so we bring it in with mod (its like the code in the curly braces is in that file)
mod front_of_house;

pub use front_of_house::hosting;

//You only need one mod declaration in your mod tree, 
//everything else can just use from absolute or relative paths (it's not like include or require)
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

