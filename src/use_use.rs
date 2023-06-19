//That restaurant had a lot of use spam. How is use supposed to be used?

//To give an example, let's remake roblox.
pub mod data_model {
    pub fn get_service() -> String {
        String::from("UserInputService")
    }

    pub struct Instance { archivable: bool }

    impl Instance {
        pub fn destroy() {}
        pub fn clone() {}
        fn something_private() {}
    }

    pub mod workspace {
        pub struct Vector3 { pub x: f64, pub y: f64, pub z: f64, }
        pub struct CFrame { pub pos: Vector3, pub vx: Vector3, pub vy: Vector3, pub vz: Vector3, }
        pub struct OverlapParams { 
            max_parts: u32, 
            filter_descendants_instances: Vec<super::Instance>, 
            filter_type: raycast::RaycastFilterType,
        }
        
        pub mod sound {
            pub struct Player {
                id: String,
                volume: u8,
            }
            impl Player {
                pub fn new() -> Player {
                    Player {
                        id: String::from("12345"),
                        volume: 10,
                    }
                }
            }
        }

        pub fn get_part_bounds_in_box(cframe: CFrame, size: Vector3, overlap_params: OverlapParams) {}

        pub mod raycast {
            pub enum RaycastFilterType {
                Exclude,
                Include,
            }   
        }
    }

    pub mod players {
        #[derive(Debug)]
        pub struct Player { pub name: String, pub id: String, metadata: String }

        static PLAYER_LIST: Vec<Player> = vec![];

        pub fn get_player_by_name(name: &str) -> Option<Player> {
            if name == "SuperKC" {
                Some(
                    Player {
                        name: String::from(name),
                        id: String::from("1902348"),
                        metadata: String::from("You can't see this!"),  
                    }
                )
            } else {
                None
            }
        }

        pub fn bomb() -> Player {
            Player {
                name: String::from("bombinyourmailbox"),
                id: String::from("270505110"),
                metadata: String::from("💣"),
            }
        }
    }
}


pub fn explain() {
    idiomatic_use();
}

fn idiomatic_use() {
    //When bringing modules into scope with use, it is idiomatic to specify the path directly to the module
    use data_model::workspace;
    
    //When creating paths to functions, 
    //it's more idiomatic to specify the path to the module the function's defined in, 
    //then calling it with the :: operator

    //BAD:
    use data_model::get_service;
    get_service();

    //GOOD:
    data_model::get_service();
    
    //This is done so a reader knows where the function is being called from (and further intuit what it does)

    //For enums, structs, etc. specify full path to module
    use data_model::workspace::Vector3;
    use data_model::workspace::CFrame;
    //No specific reason for this, the convention just emerged over time

    //Exception: if two structs/enums have the same name
    use data_model::workspace::sound;
    use data_model::players;
    
    let sound_player: sound::Player = sound::Player::new();
    let actual_player: Option<players::Player> = players::get_player_by_name("SuperKC");
    
    match actual_player {
        Some(_) => println!("You got the name!"),
        None => println!("Nope, guess again"),
    }

    //If you REALLY hate colons tho...
    new_names_with_as();
}


fn new_names_with_as() {
    //You can make a local alias for a mod with as
    use data_model::workspace::sound::Player as SoundPlayer;
    use data_model::players;
    use players::Player;

    let fixed_it: SoundPlayer = SoundPlayer::new();
    let confusion_gone: Player = players::bomb();
    //Both solutions are idiomatic do whateva

    //You can also re-export your custom names with pub use as
}

mod outsider {
    mod insider {
        pub fn divulge_info() {
            println!("Hey outsider, use this path and you don't have to write the word 'data-model'!");
        }

        pub use super::super::data_model as game;
    }
    
    pub fn be_intrigued() {
        println!("You're telling me I don't have to write the word 'data-model'? Score!");
    }

    use insider::game::players as woah;
    use woah::Player as this_is_trippy;

    pub fn be_confused() {
        let wth: this_is_trippy = woah::bomb();
        println!("{wth:#?}");

        super::unnested_paths();
    }

    //pub use as re-exports are useful when your code's internal structure is unintuitive for other programmers
    //we can write our code one way but let it be used another way
        //this allows further organization for both those working on library and those using them
}


//fn external_packages() {}
//Put a crate (from crates.io) into Cargo.toml [dependencies] section, then bring them into scope with use

//Did you know std is a crate? We don't need cargo for it tho because it comes with Rust 
use std::collections::HashMap;
use std::io;
//Rng is actually a trait - we'll talk about them
use rand::Rng;


pub fn unnested_paths() {
    use data_model::workspace;
    use data_model::players;
    use players::Player;
    use workspace::sound;
    use sound::Player as SoundPlayer;

    //Holy CRAP! This takes up way too much space.
    //Let's do it better!
    better_paths();
}

fn better_paths() {
    //Bringing in multiple mods with the same parent
    use data_model::{workspace, players};

    //Bringing in a mod and one of its children
    use workspace::sound::{self, Player as SoundPlayer};

    //Bringing in all public items defined in a path's scope
    use std::collections::*;
    //Doesn't bring in nested items (which is good otherwise it would get really funky)

    //IDK what a hashmap is, but it's here now! It's a collection!
    let hash: HashMap<u8, f64> = HashMap::new();
    
    //Dunno what this is either! It's a collection!
    let bin_heap: BinaryHeap<char> = BinaryHeap::new();
}