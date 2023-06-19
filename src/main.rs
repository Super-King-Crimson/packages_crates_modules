#![allow(unused, dead_code)]

mod organization;
mod packages_crates;
mod modules;
mod mod_name;
mod use_use;
mod separating_mods;
use separating_mods::explain as separating_mods;

fn main() {
    // organization::introduce();
    // packages_crates::explain();
    // modules::explain();
    // paths::explain();
    // use_use::explain();
    separating_mods();
}
