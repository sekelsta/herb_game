use std::io::{self, BufRead, Write};
use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

mod alchemy;

use crate::alchemy::*;

static mut WORLD: Lazy<World> = Lazy::new(World::new);

#[derive(Clone, Copy, Debug, Enum, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

#[derive(Clone, Copy, Debug, Enum, PartialEq)]
pub enum RegionEnum {
    Hut,
    Garden,
    Village,
    Field,
    FriendlyForest,
    WildflowerMeadow,
    PineForest,
    ForestRiver,
    MeadowRiver,
    //DragonMountain,
}

pub struct Region {
    name: &'static str,
    description: &'static str,
    routes: EnumMap<Direction, RegionEnum>,
    //herbs:,
}

pub struct World {
    pub regions: EnumMap<RegionEnum, Region>,
    pub current_region: RegionEnum,
    pub satchel: Vec<Ingredient>,
    pub empty_bottles: i32,
    pub infusion_shelf: Vec<Ingredient>,
    pub cauldron: Option<Ingredient>,
}

impl World {
    pub fn new() -> Self {
        use Direction::*;
        use RegionEnum::*;
        let regions = enum_map!(
            Hut => Region {
                name: "Home Sweet Home",
                description: "A simple hut with a cauldron and rack of drying herbs in the back.",
                routes: enum_map!(
                    East => Garden,
                    South | Southeast | Southwest | West => FriendlyForest,
                    North | Northeast | Northwest => Village,
                ),
            },
            Garden => Region {
                name: "Your Garden",
                description: "You can plant seeds here.",
                routes: enum_map!(
                    West => Hut,
                    East | Northeast => PineForest,
                    South | Southeast | Southwest => FriendlyForest,
                    North | Northwest => Village,
                ),
            },
            FriendlyForest => Region {
                name: "Friendly Forest",
                description: "Dapples of light filter though the soft leaves.",
                routes: enum_map!(
                    North => Hut,
                    Northwest => Garden,
                    South | Southeast | Southwest => ForestRiver,
                    East => MeadowRiver,
                    Northeast => WildflowerMeadow,
                    West => PineForest,
                ),
            },
            Village => Region {
                name: "Village Square",
                description: "You can buy or sell things here.",
                routes: enum_map!(
                    South => Hut,
                    North | Northeast | Northwest => Field,
                    East => WildflowerMeadow,
                    Southeast => MeadowRiver,
                    West | Southwest => PineForest,
                ),
            },
            Field => Region {
                name: "Weedy Field",
                description: "An overgrown farm field. The farmer says you can have the weeds for free.",
                routes: enum_map!(
                    South | Southeast | Southwest => Village,
                    East | Northeast | North => PineForest,
                    West | Northwest => WildflowerMeadow,
                ),
            },
            PineForest => Region {
                name: "Pine Forest",
                description: "Soft needles crackle beneath your feet.",
                routes: enum_map!(
                    East | Northeast => Field,
                    South | Southeast => FriendlyForest,
                    _ => PineForest,
                ),
            },
            WildflowerMeadow => Region {
                name: "Wildflower Meadow",
                description: "Tall grass for haying, interrupted by colorful flowers.",
                routes: enum_map!(
                    West | Southwest => Field,
                    South | Southeast | East => MeadowRiver,
                    _ => WildflowerMeadow,
                ),
            },
            MeadowRiver => Region {
                name: "Meadow Riverbank",
                description: "A river flows beside the meadow, the bright sun sparkling off its waters.",
                routes: enum_map!(
                    North => WildflowerMeadow,
                    Northwest => Village,
                    West => FriendlyForest,
                    South | Southwest => ForestRiver,
                    _ => MeadowRiver,
                ),
            },
            ForestRiver => Region {
                name: "Forest Riverbank",
                description: "A river flows through the forest.",
                routes: enum_map!(
                    North | Northwest | West => FriendlyForest,
                    Northeast | East => MeadowRiver,
                    _ => ForestRiver,
                ),
            },
        );

        World {
            regions,
            current_region: Hut,
            empty_bottles: 4,
            satchel: Vec::new(),
            infusion_shelf: Vec::new(),
            cauldron: None,
        }
    }

    fn has_cauldron(&self) -> bool {
        return self.current_region == RegionEnum::Hut;
    }

    fn take_ingredient(&mut self, params: &str) -> Result<Ingredient, String> {
        if params == "" {
            if self.has_cauldron() {
                let c = self.cauldron.take();
                return match c {
                    Some(ingredient) => Ok(ingredient),
                    None => Err("The cauldron is empty".to_string()),
                }
            }
        }
        // TODO: Remove and return specified ingredient from satchel
        Err("You have no such ingredient.".to_string())
    }

    fn bottle(&mut self, ingredient: &mut Ingredient) -> Result<String, String> {
        match ingredient.container {
            Container::Bottle => Err(format!("The {} is already bottled.", ingredient.name())),
            Container::None => {
                if self.empty_bottles <= 0 {
                    return Err("You don't have an empty glass bottle. Buy more bottles, or use or sell your potions.".to_string());
                }
                self.empty_bottles -= 1;
                let result = format!("You put the {} into a clean bottle.", ingredient.name());
                ingredient.container = Container::Bottle;
                Ok(result)
            }
        }
    }

    fn stir(&mut self) -> String {
        if self.has_cauldron() { match &mut self.cauldron {
            Some(ingredient) => ingredient.boil(),
            None => "The cauldron is empty.".to_string(),
        }} else {
            "You see nothing to stir.".to_string()
        }
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
        if !self.has_cauldron() {
            return "You don't have the equipment to brew potions out here.".to_string();
        }
        match &self.cauldron {
            Some(work) => "brewing not yet implemented".to_string(),
            None => {
                let work = WATER.clone();
                let descr = work.to_string();
                self.cauldron = Some(work);
                format!("You pour water into the cauldron.\n{}", descr)
            }
        }
    }

    fn look(&mut self, params: &str) -> String {
        let region = &self.regions[self.current_region];
        format!("{}\n{}", region.name, region.description)
    }

    fn help(&mut self, params: &str) -> String {
        "help text not yet implemented".to_string()
    }
}

#[wasm_bindgen]
pub fn welcome() -> String {
    "The sun shines through the aged hut's shutters as you wake up. You begin to roll over, then remember what day it is. Today is the day you're opening your very own alchemy shop!".to_string()
}

#[wasm_bindgen]
pub fn step(command: &str) -> String {
    // Not safe for multiple threads, but the program is already constrained to single-threaded for browser compatibility
    unsafe {
        let world = &mut *(&raw mut WORLD);
        let mut words = command.split_whitespace();
        let verb = match words.next() {
            Some(v) => v,
            None => return "".to_string(),
        };
        let params = words.collect::<Vec<&str>>().join(" ");
        match verb {
            "brew" => world.decoct(&params),
            "bottle" => {
                match world.take_ingredient(&params) {
                    Ok(mut i) => {
                        let result = world.bottle(&mut i);
                        world.satchel.push(i);
                        result.unwrap_or_else(|e| e)
                    }
                    Err(e) => e,
                }
            },
            "stir" => world.stir(),
            "look" => world.look(&params),
            "help" => world.help(&params),
            _ => format!("You're not sure how to '{}'. Try 'help'.", verb),
        }
    }
}
