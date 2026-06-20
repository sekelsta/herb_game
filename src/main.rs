use std::collections::HashMap;
use std::io::{self, BufRead, Write};

mod alchemy;

use crate::alchemy::*;

struct Location {
    name: &'static str,
    description: &'static str,
    //paths:,
    //herbs:,
}

struct World {
    pub locations: HashMap<&'static str, Location>,
    pub current_location: &'static str,
    pub satchel: Vec<Ingredient>,
    pub empty_bottles: i32,
    pub infusion_shelf: Vec<Ingredient>,
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
            empty_bottles: 4,
            satchel: Vec::new(),
            infusion_shelf: Vec::new(),
            cauldron: None,
        }
    }

    //fn get_ingredient(&self, params: &str) -> Result<&mut Ingredient, String> {
        // TODO: Return specified ingredient from satchel or cauldron
        //Err("You have no such ingredient".to_string())
    //}

    fn bottle(&mut self, ingredient: &mut Ingredient) -> Result<String, String> {
        if self.empty_bottles <= 0 {
            return Err("You don't have an empty glass bottle. Buy more bottles, or use or sell your potions.".to_string());
        }
        self.empty_bottles -= 1;
        let result = format!("You put the {} into a clean bottle.", ingredient.name());
        ingredient.container = Container::Bottle;
        Ok(result)
    }

    fn infuse(&mut self, base: &mut Ingredient, addition: &mut Ingredient) -> String {
        match base.container {
            Container::Bottle => base.infuse(addition),
            Container::None => match self.bottle(base) {
                Ok(result) => (),
                Err(result) => return result,
            },
        };
        let mut infusion = base.clone();
        let mut ingredient = addition.clone();
        // TODO: Filter out elements by infusion base type (water, tincture, oil)
        ingredient.halve();
        infusion.add(ingredient);
        let result = infusion.to_string();
        self.infusion_shelf.push(infusion);
        result
    }

    fn decoct(&mut self, params: &str) -> String {
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
        "brew" => world.decoct(&params),
        //"bottle" => world.get_ingredient(&params).map_or_else(|e| e, |i| world.bottle(&mut i).unwrap()),
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
