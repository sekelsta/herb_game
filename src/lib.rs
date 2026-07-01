use std::str::FromStr;
use rand::RngExt;
use std::cell::RefCell;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

mod alchemy;
mod elements;
mod herbs;
mod knowledge;
mod potions;
mod region;
mod world;

use crate::potions::Effect;
use crate::alchemy::*;
use crate::elements::*;
use crate::region::*;
use crate::knowledge::*;
use crate::herbs::*;
use crate::world::World;

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::new());
}

fn help() -> String {
"
north/south/east/west or the name of a neighboring location - to travel
look - describe your current location, including cauldron contents
map - display a map of the area

gather - search for herbs in your current region
inv or satchel - list items inside your satchel
herbs - lists where you've found each herb before

book, note, letter, infusions - read about how to do alchemy, unlocked over time
brew [ingredient] - add the ingredient to the cauldron for a decoction
soak [ingredient] - add the ingredient to a bottle for an infusion. Not available in early game.
stir - stir the cauldron as it boils, allowing lighter elements to evaporate
bottle [ingredient] - put the named ingredient into a bottle, or finish and bottle what's brewing in the cauldron
dump - empty out the cauldron and get rid of the contents
recipes - check your notes on recipes you've discovered

sleep - advances time, allowing herbs to regrow, infusions to infuse, and fresh herbs to dry out
buy/sell [item] - exchange goods for money at the village market
xp - tells you how close you are to learning something new
close shop - delete your save and start the game over again".to_string()
}

#[wasm_bindgen]
pub fn welcome() -> String {
    "The sun shines through the aged hut's shutters as you wake up. You begin to roll over, then remember what day it is. Today is the day you're opening your very own alchemy shop!\nYou remember your trip in yesterday from the NORTH, passing through the VILLAGE. Just north of the village was a field with a few plants you recognized, perfect for starting some experiments on.\nSitting on the table is your introductory BOOK on alchemy.\nType a direction to travel, or 'help' to list commands.".to_string()
}

#[wasm_bindgen]
pub fn welcome_on_load() -> String {
    "Welcome back to your alchemy shop. Type 'help' if you get stuck.".to_string()
}

#[wasm_bindgen]
pub fn step(command: &str) -> String { WORLD.with_borrow_mut(|world| {
    let mut words = command.split_whitespace();
    let verb = match words.next() {
        Some(v) => v,
        None => return "".to_string(),
    };
    match Direction::from_str(verb) {
        Ok(direction) => return world.travel_cardinal(direction),
        Err(_) => (),
    };
    match RegionEnum::from_str(command) {
        Ok(region) => return world.travel_to(region),
        Err(_) => (),
    };
    let params = words.collect::<Vec<&str>>().join(" ");
    match verb {
        "go"|"travel"|"to"|"the" => step(&params),
        "wait"|"advance"|"sleep" => world.advance_time(),
        "inv"|"inventory"|"satchel"|"list" => world.list_inventory(),
        "gather"|"forage"|"collect" => world.forage(rand::rng().random_range(1..4)),
        "herb"|"herbs" => world.discoveries.list_herb_locations(),
        "recipe"|"recipes"|"effects" => world.discoveries.list_recipes(),
        "brew"|"decoct"|"cauldron" => world.decoct_named(&params),
        "soak"|"infuse" => world.infuse_named(&params),
        "bottle" => world.bottle_named(&params),
        "dump"|"spill"|"empty" => world.dump(&params),
        "stir" => world.stir(),
        "sell" => world.sell(&params),
        "buy" => world.buy(&params),
        "market" => world.market_info(),
        "exp"|"xp"|"status" => world.experience(),
        "map" | "surroundings" => world.regions[world.current_region].local_map(),
        "book"|"textbook"|"alchemy" => world.discoveries.book(),
        "note"|"experiement"|"experiments" => world.discoveries.show_stability_note(),
        "letter" => world.discoveries.show_boost_note(),
        "infusion"|"infusions" => world.discoveries.show_infusion_instructions(),
        "look" => world.look(),
        "help" => help(),
        _ => format!("You're not sure how to '{}'. Try 'help'.", verb),
    }
})}

#[wasm_bindgen]
pub fn save_to_json() -> String {
    WORLD.with(|world|
        serde_json::to_string(world).unwrap()
    )
}

#[wasm_bindgen]
pub fn load_from_json(json: &str) {
    WORLD.set(serde_json::from_str(json).unwrap());
}

#[wasm_bindgen]
pub fn restart_save() {
    WORLD.set(World::new());
}
