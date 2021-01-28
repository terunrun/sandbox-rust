#![allow(dead_code)] // この行でコンパイラのwaringsメッセージを止めます。

enum Species { Crab, Octopus, Fish, Clam }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        // species: Species::Octopus,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
        // weapon: Weapon::Poison(PoisonType::Acidic),
    };

    match ferris.species {
        // enumでSpeciesを定義しているのなら、以下のように列挙するのでなくてもっとすっきり書けないものか？
        Species::Crab => println!("{} is a crab",ferris.name),
        Species::Octopus => println!("{} is a octopus",ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        Species::Clam => println!("{} is a clam",ferris.name),
    }

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    // sizeフィールドがBigならbigを、Smallならsmallをsize_descriptionにバインド
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }
}
