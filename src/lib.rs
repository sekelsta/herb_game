use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;
use std::str::FromStr;
use strum_macros::EnumString;
use wasm_bindgen::prelude::*;

mod alchemy;
mod herbs;

use crate::alchemy::*;
use crate::herbs::*;

static mut WORLD: Lazy<World> = Lazy::new(World::new);

const MAP: &str = r#"Surroundings:
PPPPPPPPPPPPPPPPPwPPPwwwwwwwwwwwwwww
PPPPPPPPPPPPPPPPPPwPPPwwwwwwwwwwwwww
PPPPPPPPPP----x---------wwwwwwwwwwww
PPPPPPPPPP----------x---wwwwwwwwwwww
PPPPPPPPPP---x----------wwwwwwwwwwww
PPPPPPPPPP     _     wwwwwwwwwwwwwww
PPPPPPP P     / \      wwwwwwwwwwwww
PPPPPPPP   _  | |  _      wwwwwwwwww
PDPPDPPP  / \     / \  wwwwwwwwwwwww
PPDPPPDPD | |  __ | |    wwwwwwwwww/
PDPPDPPDDP              wwwwwwwwww//
PDPDPPDDPD       D _   DwwDwwwDw//ww
DPDDPDDD  |~~~~|  / \   wwwwDDw//www
DDDPDDPDD |~~~~|  |H| DDwDDwD//DDwDw
DDDDDDDDDDDD      DDDDDDDD/~/wDDwDDw
DDDDDDDDDDDDDDDDDDDDDDD//D//DDDDDDDw
DDDDDDDDDDDDDDDDDDDD//DDD//DDDDDDDDD
DDDDDDDDDDDDDDDD//DDDDDD//DDDDDDDDDD
DDDDDDDDDDDDDD//DDDDDDD||DDDDDDDDDDD
DDDDDDDDDDDD//DDDDDDDDDD\\DDDDDDDDDD"#;

#[derive(Clone, Copy, Debug, Enum, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
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

#[derive(Clone, Copy, Debug, Enum, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum RegionEnum {
    Hut,
    Garden,
    #[strum(serialize = "village square", serialize = "village", serialize = "market")]
    Village,
    #[strum(serialize = "weedy field", serialize = "field", serialize = "overgrown field")]
    Field,
    #[strum(serialize = "friendly forest", serialize = "forest")]
    FriendlyForest,
    #[strum(serialize = "wildflower meadow", serialize = "meadow")]
    WildflowerMeadow,
    #[strum(serialize = "pine forest", serialize = "pines")]
    PineForest,
    #[strum(serialize = "forest river")]
    ForestRiver,
    #[strum(serialize = "meadow river")]
    MeadowRiver,
}

pub struct Region {
    name: &'static str,
    description: &'static str,
    routes: EnumMap<Direction, RegionEnum>,
    current_herbs: Vec<Ingredient>,
    possible_herbs: Vec<&'static Ingredient>,
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
                current_herbs: Vec::new(),
                possible_herbs: Vec::new(),
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
                current_herbs: Vec::new(),
                possible_herbs: Vec::new(),
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
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*NEW_YORK_FERN, &*VIOLET, &*JACK_IN_THE_PULPIT, &*BLUEBELL, &*TROUT_LILY, &*WILD_STRAWBERRY, &*ENCHANTERS_NIGHTSHADE, &*BURDOCK),
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
                current_herbs: Vec::new(),
                possible_herbs: Vec::new(),
            },
            Field => Region {
                name: "Weedy Field",
                description: "An overgrown farm field. The farmer says you can have the weeds for free.",
                routes: enum_map!(
                    South | Southeast | Southwest => Village,
                    East | Northeast | North => PineForest,
                    West | Northwest => WildflowerMeadow,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*DANDELION, &*PURSLANE, &*PETTY_SPURGE, &*WHITE_CLOVER, &*VELVETLEAF, &*HORSETAIL, &*HEALALL, &*YARROW, &*FLEABANE, &*BLACK_NIGHTSHADE),
            },
            PineForest => Region {
                name: "Pine Forest",
                description: "Soft needles crackle beneath your feet.",
                routes: enum_map!(
                    East | Northeast => Field,
                    South | Southeast => FriendlyForest,
                    _ => PineForest,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*VIOLET, &*DAFFODIL, &*COLUMBINE, &*WHITE_TRILLIUM, &*LADY_FERN, &*WINTERGREEN, &*YEW, &*DEADLY_NIGHTSHADE),
            },
            WildflowerMeadow => Region {
                name: "Wildflower Meadow",
                description: "Tall grass for haying, interrupted by colorful flowers.",
                routes: enum_map!(
                    West | Southwest => Field,
                    South | Southeast | East => MeadowRiver,
                    _ => WildflowerMeadow,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*BUTTERCUP, &*RED_CLOVER, &*OXEYE_DAISY, &*BULL_THISTLE, &*MILKWEED, &*HEALALL, &*SWEET_ANNIE, &*YARROW, &*POISON_HEMLOCK, &*PASTURE_ROSE, &*FEVERFEW, &*CHAMOMILE, &*BORAGE, &*YELLOW_DOCK),
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
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*WATERMINT, &*TURTLEHEAD, &*JOE_PYE, &*MEADOW_ANEMONE, &*HORSETAIL, &*COLTSFOOT, &*WILLOW, &*MARSH_MALLOW),
            },
            ForestRiver => Region {
                name: "Forest Riverbank",
                description: "A river flows through the forest.",
                routes: enum_map!(
                    North | Northwest | West => FriendlyForest,
                    Northeast | East => MeadowRiver,
                    _ => ForestRiver,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*JEWELWEED, &*FOX_SEDGE, &*SKUNK_CABBAGE, &*CINNAMON_FERN, &*MEADOWSWEET, &*SPOTTED_DEADNETTLE, &*COLTSFOOT),
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

    fn travel_cardinal(&mut self, direction: Direction) -> String {
        let current = self.current_region;
        self.current_region = self.regions[self.current_region].routes[direction];
        return if current == self.current_region {
            "You decide not to travel so far from home after all.".to_string()
        } else {
            self.look()
        }
    }

    fn travel_to(&mut self, region: RegionEnum) -> String {
        if region == self.current_region {
            "You're already here.".to_string()
        } else if self.regions[self.current_region].routes.values().any(|r| *r == region) {
            self.current_region = region;
            self.look()
        } else {
            "That's not nearby.".to_string()
        }
    }

    fn has_cauldron(&self) -> bool {
        return self.current_region == RegionEnum::Hut;
    }

    fn list_satchel(&self) -> String {
        self.satchel.iter().map(|i| i.full_name()).collect::<Vec<String>>().join("\n")
    }

    fn take_ingredient(&mut self, params: &str) -> Result<Ingredient, String> {
        if params == "" {
            if self.has_cauldron() {
                let c = self.cauldron.take();
                return match c {
                    Some(ingredient) => Ok(ingredient),
                    None => Err("The cauldron is empty".to_string()),
                }
            };
            return Err("Specify an ingredient".to_string())
        }
        if let Some(pos) = self.satchel.iter().position(|x| x.matches_name(params)) {
            return Ok(self.satchel.remove(pos));
        }
        if let Some(pos) = self.infusion_shelf.iter().position(|x| x.full_name() == params) {
            return Err("Wait for that to finish infusing first.".to_string()) 
        }
        Err(format!("You have no such ingredient: {}", params)) 
    }

    fn bottle(&mut self, ingredient: &mut Ingredient) -> Result<String, String> {
        match ingredient.container {
            Container::Bottle => Err(format!("The {} is already bottled.", ingredient.full_name())),
            Container::None => {
                if self.empty_bottles <= 0 {
                    return Err("You don't have an empty glass bottle. Buy more bottles, or use or sell your potions.".to_string());
                }
                self.empty_bottles -= 1;
                let result = format!("You put the {} into a clean bottle.", ingredient.full_name());
                ingredient.container = Container::Bottle;
                Ok(result)
            }
        }
    }

    fn dump_cauldron(&mut self) -> String {
        if !self.has_cauldron() {
            return "There's no cauldron to dump here.".to_string();
        }
        match &self.cauldron {
            Some(work) => {
                let descr = work.to_string();
                self.cauldron = None;
                format!("Dumped from cauldron: {}", descr)
            },
            None => "The cauldron is already empty".to_string(),
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
                let name = work.full_name();
                self.cauldron = Some(work);
                format!("You pour {0} into the cauldron and bring it to a boil.\n{1}", name, descr)
            }
        }
    }

    fn look(&mut self) -> String {
        let region = &self.regions[self.current_region];
        format!("{}\n{}", region.name, region.description)
    }

    fn help(&mut self) -> String {
"TODO - potion making instructions
north, south, east, west, [location name] - travel
inv or satchel - list items inside your satchel
bottle [ingredient] - put the named ingredient into a bottle, or finish and bottle what's brewing in the cauldron
dump - empty out the cauldron and get rid of the contents
map - display a map of the area
look - describe your current location
help - print this info".to_string()
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
            "inv"|"inventory"|"satchel" => world.list_satchel(),
            "brew"|"decoct"|"cauldron" => world.decoct(&params),
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
            "dump"|"spill" => world.dump_cauldron(),
            "stir" => world.stir(),
            "map" | "surroundings" => MAP.to_string(),
            "look" => world.look(),
            "help" => world.help(),
            _ => format!("You're not sure how to '{}'. Try 'help'.", verb),
        }
    }
}
