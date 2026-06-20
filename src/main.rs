use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead, Write};
use once_cell::sync::Lazy;

use enum_map::{enum_map, Enum, EnumMap};

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

#[derive(Debug, Default)]
struct ElementStatus {
    pub power: i32,
    pub stability: i32,
}

#[derive(Debug)]
enum PotionKind {
    Decoction, // Boil for herbal tea
    Infusion, // Cold soak
    Tincture, // Soak in alcohol
    Oil, // Or essence. Mild heat to speed up extraction, or cold to preserve aromatics
    Salve, // Add wax to the oil
    Poultice,
    Incense, // For resins, or Smudge for leaves/flowers
    Smudge,
    Salt
}

impl fmt::Display for PotionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Work {
    kind: PotionKind,
    elements: EnumMap<Element, ElementStatus>,
}

impl Work {
    fn new_in_cauldron() -> Self {
        let mut elements: EnumMap<Element, ElementStatus> = EnumMap::default();
        elements[Element::Water].power = 4;
        Work {
            kind: PotionKind::Decoction,
            elements,
        }
    }

    fn tick(&mut self) {
        match self.kind {
            PotionKind::Decoction => {
                // One air evaporates, if present
                self.elements[Element::Air].power = (self.elements[Element::Air].power - 1).max(0);
                // Taint spreads
                let tainted = self.elements[Element::Taint].power;
                for _ in 0..tainted {
                    let mut most = TAINTABLE_ELEMENTS[0];
                    for e in TAINTABLE_ELEMENTS {
                        if self.elements[e].power > self.elements[most].power {
                            most = e;
                        }
                    }
                    if self.elements[most].power > 0 {
                        self.elements[most].power -= 1;
                        self.elements[Element::Taint].power += 1;
                    }
                }
            },
            _ => todo!(),
        }
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

    fn occupies_bottle(&self) -> bool {
        match self.kind {
            PotionKind::Decoction | PotionKind::Infusion | PotionKind::Tincture | PotionKind::Oil | PotionKind::Salve | PotionKind::Poultice => true,
            PotionKind::Smudge | PotionKind::Incense | PotionKind::Salt => false,
        }
    }
}

impl fmt::Display for Work {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.kind)?;
        let any = false;
        for (element, status) in self.elements.iter().filter(|(_, s)| s.power != 0) {
            if any {
                write!(f, ", ")?;
            }
            write!(f, "{} {:?}", status.power, element)?; // TODO: Stability
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
    pub brewed_works: Vec<Work>,
    pub cauldron: Option<Work>,
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
        if self.brewed_works.iter().filter(|&n| n.occupies_bottle()).count() as i32 >= self.total_bottles {
            return "You don't have an empty glass bottle. Buy more bottles, or use or sell your potions.".to_string();
        }

        match &self.cauldron {
            Some(work) => "brewing not yet implemented".to_string(),
            None => {
                let work = Work::new_in_cauldron();
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
