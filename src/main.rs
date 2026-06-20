use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use once_cell::sync::Lazy;

use enum_map::{enum_map, Enum, EnumMap};

static DANDELION: Lazy<Ingredient> = Lazy::new(|| Ingredient { elements: vec! {
    (Element::Air, Modifier::Provide, 2),
    (Element::Fire, Modifier::Provide, 1),
    (Element::Water, Modifier::Destabilize, 1),
}});

static VIOLET: Lazy<Ingredient> = Lazy::new(|| Ingredient { elements: vec! {
    (Element::Water, Modifier::Provide, 2),
    (Element::Order, Modifier::Provide, 1),
    (Element::Chaos, Modifier::Strengthen, 1),
}});

struct Location {
    name: &'static str,
    description: &'static str,
    //paths:,
    //herbs:,
}

struct World {
   locations: HashMap<&'static str, Location>,
}

impl World {
    fn new() -> Self {
        let mut locations = HashMap::new();
        locations.insert("hut", Location {
            name: "Home Sweet Home",
            description: "A simple hut with a cauldron and rack of drying herbs in the back.",
        });

        World {
            locations,
        }
    }
}

#[derive(Clone, Copy, Debug, Enum, PartialEq)]
enum Element {
    Order,
    Chaos,
    Earth,
    Water,
    Air,
    Fire,
    Ice,
    Thunder,
    Spirit, // AKA Ether
    Mana,
    Taint,
    Void,
    Light,
    Shadow,
}

#[derive(Clone, Copy, Debug, Enum, PartialEq)]
enum Modifier {
    Strengthen,
    Reduce,
    Stabilize,
    Destabilize,
    Provide,
}

struct Ingredient {
    pub elements: Vec<(Element, Modifier, i32)>,
}

struct ElementStatus {
    pub power: i32,
    pub stability: i32,
}

struct Work {
    elements: EnumMap<Element, ElementStatus>,
}

impl Work {
    fn tick(&mut self) {
        // TODO: Apply chain reaction logic
    }

    fn add(&mut self, ingredient: Ingredient) {
        for (element, modifier, amount) in ingredient.elements {
            match modifier {
                Modifier::Strengthen => self.elements[element].power += amount.min(self.elements[element].power),
                Modifier::Reduce => self.elements[element].power = (self.elements[element].power - amount).max(0),
                Modifier::Stabilize => self.elements[element].stability += amount,
                Modifier::Destabilize => self.elements[element].stability -= amount,
                Modifier::Provide => self.elements[element].power += amount,
            }
        }
    }
}

enum HerbState {
    Fresh,
    Dry,
    Ground,
    Paste
}

enum Brew {
    Infusion, // Cold soak
    Decoction, // Boil for herbal tea
    Tincture, // Soak in alcohol
    Oil, // Or essence. Mild heat to speed up extraction, or cold to preserve aromatics
    Salve, // Add wax to the oil
    Poultice,
    Incense, // For resins, or Smudge for leaves/flowers
    Salt
}

fn brew(world: &mut World, params: &str) -> String {
    "brewing not yet implemented".to_string()
}

fn help(world: &mut World, params: &str) -> String {
    "help text not yet implemented".to_string()
}

fn step(world: &mut World, command: &str) -> String {
    let mut words = command.split_whitespace();
    let verb = match words.next() {
        Some(v) => v,
        None => return "".to_string(),
    };
    let params = words.collect::<Vec<&str>>().join(" ");
    match verb {
        "brew" => brew(world, &params),
        "help" => help(world, &params),
        _ => format!("You're not sure how to {}. Try 'help'.", verb),
    }
}

fn main() -> io::Result<()> {
    let mut world = World::new();
    println!("Dragon's Blood\n\
            The sun shines through the aged hut's shutters as you wake up. You begin to roll \
            over, then remember what day it is. Today is the day you're opening your very own alchemy shop!");
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line)?;

        // To halt on Ctrl+D
        if line.len() == 0 {
            break;
        }

        line = line.trim().to_lowercase();
        if line == "quit" {
            break;
        }

        let result = step(&mut world, &line);
        print!("{result}");
    }

    Ok(())
}
