pub mod submod_name {
    pub fn hi() {
        println!("Hello, World! (from submod)")
    }
}

pub fn hi() {
    println!("Hello, World!");
}

pub const THE_FIVE_NEW_KANJI: [char; 5] = ['夕', '前', '早', '走', '草'];
pub const GLOBS_ARE_BAD: &str = "If you use this you're dumb"; 