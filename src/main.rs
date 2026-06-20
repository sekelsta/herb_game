use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead, Write};
use once_cell::sync::Lazy;

use enum_map::{Enum, EnumMap};

const TAINTABLE_ELEMENTS: [Element; 13] = [
    Element::Mana,
    Element::Spirit,
    Element::Chaos,
    Element::Shadow,
    Element::Void,
    Element::Thunder,
    Element::Air,
    Element::Ice,
    Element::Light,
    Element::Fire,
    Element::Water,
    Element::Earth,
    Element::Order,
];

static DANDELION: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Air][Modifier::Provide] = 2;
    elements[Element::Fire][Modifier::Provide] = 1;
    elements[Element::Water][Modifier::Stabilize] = -1;
    Ingredient { kind: IngredientKind::FreshHerb, elements }
});

static CAULDRON_WATER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Water][Modifier::Provide] = 4;
    Ingredient { kind: IngredientKind::Decoction, elements }
});

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
    Strengthen, // Weaken if value is negative
    Stabilize, // Destabilize if value is negative
    Provide,
    //Join,
    //Split,
}

#[derive(Clone, Copy, Debug)]
enum IngredientKind {
    FreshHerb,
    DryHerb,
    Decoction, // Boil for herbal tea
    Infusion, // Cold soak
    Tincture, // Soak in alcohol
    Oil, // Or essence. Mild heat to speed up extraction, or cold to preserve aromatics
    Salve, // Add wax to the oil
    Poultice,
    Incense, // For resins, or Smudge for leaves/flowers
    Smudge,
    Ash,
    Salt,
}

impl IngredientKind {
    fn occupies_bottle(&self) -> bool {
        match self {
            Self::FreshHerb | Self::DryHerb | Self::Smudge | Self::Incense => false,
            Self::Decoction | Self::Infusion | Self::Tincture | Self::Oil => true,
            Self::Salve | Self::Poultice => true,
            Self::Salt | Self::Ash => true,
        }
    }
}

impl fmt::Display for IngredientKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
struct Ingredient {
    pub kind: IngredientKind,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
}

impl Ingredient {
    fn boil(&mut self) -> String {
        // One air evaporates, if present
        let air_evaporated = self.elements[Element::Air][Modifier::Provide] > 0;
        self.elements[Element::Air][Modifier::Provide] = (self.elements[Element::Air][Modifier::Provide] - 1).max(0);
        // Taint spreads
        let tainted = self.elements[Element::Taint][Modifier::Provide];
        let mut taint_spread = false;
        for _ in 0..tainted {
            let mut most = TAINTABLE_ELEMENTS[0];
            for e in TAINTABLE_ELEMENTS {
                if self.elements[e][Modifier::Provide] > self.elements[most][Modifier::Provide] {
                    most = e;
                }
            }
            if self.elements[most][Modifier::Provide] > 0 {
                self.elements[most][Modifier::Provide] -= 1;
                self.elements[Element::Taint][Modifier::Provide] += 1;
                taint_spread = true;
            }
        }
        match (taint_spread, air_evaporated) {
            (false, false) => "The cauldron boils.".to_string(),
            (false, true) => "The cauldron boils. Elemental air evaporates.".to_string(),
            (true, false) => "The cauldron boils. Taint spreads.".to_string(),
            (true, true) => "The cauldron boils. Elemental air evaporates. Taint spreads.".to_string(),
        }
    }

    fn add(&mut self, ingredient: Ingredient) {
        for (element, modifiers) in ingredient.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] += ingredient.elements[element][modifier];
            }
        }
    }

    fn halve(&mut self) {
        for (element, modifiers) in self.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] = (self.elements[element][modifier] as f32 / 2.0).ceil() as i32;
            }
        }
    }

    fn apply(&mut self, ingredient: Ingredient) {
        for (element, modifiers) in ingredient.elements {
            for (modifier, amount) in modifiers {
                let power = self.elements[element][Modifier::Provide];
                match modifier {
                    Modifier::Strengthen => self.elements[element][Modifier::Provide] = (power + amount.min(power)).max(0),
                    Modifier::Stabilize => self.elements[element][Modifier::Stabilize]+= amount,
                    Modifier::Provide => self.elements[element][Modifier::Provide] += amount,
                }
            }
        }
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.kind)?;
        let any = false;
        for (element, status) in self.elements.iter().filter(|(_, s)| s[Modifier::Provide] != 0) {
            if any {
                write!(f, ", ")?;
            }
            write!(f, "{} {:?}", status[Modifier::Provide], element)?; // TODO: Stability
        }
        Ok(())
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
    Smudge,
    Incense, // For resins, or Smudge for leaves/flowers
    Salt
}

struct Location {
    name: &'static str,
    description: &'static str,
    //paths:,
    //herbs:,
}

struct World {
    pub locations: HashMap<&'static str, Location>,
    pub current_location: &'static str,
    pub total_bottles: i32,
    pub brewed_works: Vec<Ingredient>,
    pub cauldron: Option<Ingredient>,
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
            current_location: "hut",
            total_bottles: 4,
            brewed_works: Vec::new(),
            cauldron: None,
        }
    }

    fn brew(&mut self, params: &str) -> String {
        if self.brewed_works.iter().filter(|&n| n.kind.occupies_bottle()).count() as i32 >= self.total_bottles {
            return "You don't have an empty glass bottle. Buy more bottles, or use or sell your potions.".to_string();
        }

        match &self.cauldron {
            Some(work) => "brewing not yet implemented".to_string(),
            None => {
                let work = CAULDRON_WATER.clone();
                let descr = work.to_string();
                self.cauldron = Some(work);
                format!("You pour water into the cauldron.\n{}", descr)
            }
        }
    }

    fn look(&mut self, params: &str) -> String {
        let location = &self.locations[&self.current_location];
        format!("{}\n{}", location.name, location.description)
    }

    fn help(&mut self, params: &str) -> String {
        "help text not yet implemented".to_string()
    }
}

fn step(world: &mut World, command: &str) -> String {
    let mut words = command.split_whitespace();
    let verb = match words.next() {
        Some(v) => v,
        None => return "".to_string(),
    };
    let params = words.collect::<Vec<&str>>().join(" ");
    match verb {
        "brew" => world.brew(&params),
        "look" => world.look(&params),
        "help" => world.help(&params),
        _ => format!("You're not sure how to '{}'. Try 'help'.", verb),
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
        println!("{result}");
    }

    Ok(())
}
