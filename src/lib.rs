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

const ALCHEMY_BOOK: &str = "Alchemy for Dummies
If you are just starting out, alchemy can seem quite daunting! But don't worry, it is.
There are two main procedures to know about: infusion and decoction. Both are much simpler than they sound. Infusion is soaking the ingredient in a liquid for a long time. This is typically done in a jar or bottle, left sitting in a cool dark place such as a shelf. Meanwhile decoction is a shorter soak at a much hotter temperature, that is, your standard boil-in-a-cauldron treatment witches have been practicing for aeons.
Whichever method you use, you will need to concern yourself with the elemental energies of your materials. Each desired effect requires a certain combination of elements to achieve. Some variance is allowable, but makes for a weaker potion.
So how do you actually brew a potion? Start with a liquid base, usually water but you may also use spirits or oil. Bring your cauldron to a boil, then add your herbs one at a time. Each herb you add may provide elemental energy directly, strengthen or weaken pre-existing elements from earlier additions to your brew, or affect the stability of an element. Having a lot of any single element in a potion can lead to unexpected effects. Increased stability allows for more extreme potions, while destabilizing effects can turn your potion into an elemental chain reaction even at weaker strengths. Speaking of chain reactions, watch out for taint. Any elemental taint in your brew will convert other elements into more of itself, which can quickly get out of hand. As you go certain elements may evaporate, so take care with the order and timing. Try out a practice batch and you'll understand. You can dump out your cauldron if the contents have become unsalvageable. Or if you're satisfied with what you've made, bottle it and wash your cauldron for the next batch.
Infusions are if anything simpler than decoctions. Put your herb right into a bottle with any liquid base, and leave it in a cool dark place, perhaps a shelf, while you sleep. After enough time has passed you will see that the liquid has taken on some of the color of the plant. This indicates that the plant's elemental energies have leached into the liquid, and your infusion is ready to be filtered and put to use. You can make potions directly as infusions, you can add them to your boiling decoctions in place of the herb, or you can even add another herb and infuse again. You can also infuse an herb into a decoction, with or without existing potion effects, to add to the elements and potentially create a new potion out of it. Note that infusing an herb and decocting an herb will not have quite the same effect. Only elements the herb provides directly will be available to create potion effects on the infusion. Herbs which strengthen existing elements will not do so when added to an infusion. Another thing to note is that more of the elemental energy stays with the plant compared to a decoction. Certain types of elements, if they are not soluble in your chosen base, will not become available at all. This can be used to your advantage to purify the remaining elements, and can allow for higher quality potions if you know what you're doing.
That's about all you need to know to get started. I encourage you to experiment for youself to discover what effects you can create. With trial and error you'll be able to refine your recipes, and as you go on you'll discover cheaper and more effective combinations of ingredients. Good luck!";

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
    #[strum(serialize = "hut", serialize = "home")]
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

impl Region {
    fn regrow(&mut self) {
        for i in (0..self.current_herbs.len()).rev() {
            if rand::random_bool(0.5) {
                self.current_herbs.remove(i);
            }
        }
        for &h in &self.possible_herbs {
            if rand::random_bool(0.1) {
                self.current_herbs.push(h.clone());
            }
            if rand::random_bool(0.1) {
                self.current_herbs.push(h.clone());
            }
        }
        // TODO: Shuffle so herbs are not always found in the same order
    }
}

pub struct World {
    pub regions: EnumMap<RegionEnum, Region>,
    pub current_region: RegionEnum,
    pub satchel: Vec<Ingredient>,
    pub unlimited_ingredients: Vec<&'static Ingredient>,
    pub empty_bottles: i32,
    pub bottles_sold: i32,
    pub money: i32,
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
                    West => Garden,
                    South | Southeast | Southwest | East => FriendlyForest,
                    North | Northeast | Northwest => Village,
                ),
                current_herbs: Vec::new(),
                possible_herbs: Vec::new(),
            },
            Garden => Region {
                name: "Your Garden",
                description: "Needs some work. You decide you'd rather keep foraging from the wild.\nA few herbs have survived from better-tended times, with occasional weeds sprouting around them.",
                routes: enum_map!(
                    East => Hut,
                    West | Northwest => PineForest,
                    South | Southeast | Southwest => FriendlyForest,
                    North | Northeast => Village,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!( &*DANDELION, &*FEVERFEW, &*DAFFODIL),
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
                    West => FriendlyForest,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*VIOLET, &*ENCHANTERS_NIGHTSHADE, &*BLUEBELL),// &*JACK_IN_THE_PULPIT, &*TROUT_LILY, &*WILD_STRAWBERRY, &*NEW_YORK_FERN, &*BURDOCK),
            },
            Village => Region {
                name: "Village Square",
                description: "You can buy or sell things here.\nEmpty bottles cost 1 silver apiece, spirits cost 8 and oil costs 24.",
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
                    East | Northeast | North => WildflowerMeadow,
                    West | Northwest => PineForest,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*HORSETAIL, &*DANDELION, &*PURSLANE, &*PETTY_SPURGE, &*WHITE_CLOVER),// &*VELVETLEAF, &*HEALALL, &*YARROW, &*FLEABANE, &*BLACK_NIGHTSHADE, &*BITTERSWEET),
            },
            PineForest => Region {
                name: "Pine Forest",
                description: "Soft needles crackle beneath your feet.",
                routes: enum_map!(
                    East | Northeast => Field,
                    South | Southeast => Village,
                    _ => PineForest,
                ),
                current_herbs: Vec::new(),
                possible_herbs: vec!(&*VIOLET, &*COLUMBINE, &*WINTERGREEN),// &*WHITE_TRILLIUM, &*LADY_FERN, &*YEW, &*DEADLY_NIGHTSHADE),
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
                possible_herbs: vec!(&*BUTTERCUP, &*RED_CLOVER, &*OXEYE_DAISY, &*BULL_THISTLE, &*MILKWEED, &*FEVERFEW),// &*HEALALL, &*SWEET_ANNIE, &*YARROW, &*POISON_HEMLOCK, &*PASTURE_ROSE, &*CHAMOMILE, &*BORAGE, &*YELLOW_DOCK),
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
                possible_herbs: vec!(&*WATERMINT, &*HORSETAIL, &*COLTSFOOT),// &*TURTLEHEAD, &*JOE_PYE, &*MEADOW_ANEMONE, &*WILLOW, &*MARSH_MALLOW),
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
                possible_herbs: vec!(&*JEWELWEED, &*SPOTTED_DEADNETTLE, &*COLTSFOOT, &*FOX_SEDGE, &*SKUNK_CABBAGE),// &*CINNAMON_FERN, &*MEADOWSWEET),
            },
        );

        let mut world = World {
            regions,
            current_region: Hut,
            empty_bottles: 4,
            bottles_sold: 0,
            money: 2,
            unlimited_ingredients: vec!(&*WATER, &*ROT),
            satchel: Vec::new(),
            infusion_shelf: Vec::new(),
            cauldron: None,
        };
        world.advance_time();
        world
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

    fn list_inventory(&self) -> String {
        format!("{0}\nEmpty glass bottles: {1}\nSilver pieces: {2}", self.list_satchel(), self.empty_bottles, self.money)
    }

    fn list_satchel(&self) -> String {
        self.satchel.iter().map(|i| i.inventory_view()).collect::<Vec<String>>().join("\n")
    }

    fn forage(&mut self, params: &str) -> String {
        let available = &mut self.regions[self.current_region].current_herbs;
        if available.len() == 0 {
            return "You found nothing.".to_string();
        }
        let pos = available.iter().position(|x| x.matches_name(params)).or(Some(0)).unwrap();
        let found = available.remove(pos);
        let result = format!("You collected {}.", found.full_name());
        self.satchel.push(found);
        result        
    }

    fn take_ingredient(&mut self, params: &str, filter: impl Fn(&Ingredient) -> bool) -> Result<Ingredient, String> {
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
        if let Some(pos) = self.satchel.iter().position(|x| filter(x) && x.matches_name(params)) {
            return Ok(self.satchel.remove(pos));
        }
        if let Some(pos) = self.unlimited_ingredients.iter().position(|x| x.matches_name(params)) {
            return Ok(self.unlimited_ingredients[pos].clone());
        }
        if let Some(_pos) = self.infusion_shelf.iter().position(|x| x.full_name() == params) {
            return Err("Wait for that to finish infusing first.".to_string())
        }
        Err(format!("You have no such ingredient: {}", params))
    }

    fn bottle_named(&mut self, params: &str) -> String {
        if params == "bottle" {
            return "Nice try. You can't fit a bottle inside a bottle.".to_string();
        }
        match self.take_ingredient(&params, |ingr: &Ingredient| ingr.container == Container::None) {
            Ok(mut ingr) => {
                let result = self.bottle(&mut ingr);
                self.satchel.push(ingr);
                result.unwrap_or_else(|e| e)
            }
            Err(e) => e,
        }
    }

    fn bottle(&mut self, ingredient: &mut Ingredient) -> Result<String, String> {
        match ingredient.container {
            Container::Bottle => Err(format!("The {} is already bottled.", ingredient.full_name())),
            Container::None => {
                if self.empty_bottles <= 0 {
                    return Err("You don't have an empty glass bottle. Buy more bottles, or sell your potions. Customers may or may not return the empty bottle afterwards.".to_string());
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
                let descr = work.show_in_progress();
                self.cauldron = None;
                format!("Dumped from cauldron: {}", descr)
            },
            None => "The cauldron is already empty".to_string(),
        }
    }

    fn dump(&mut self, params: &str) -> String {
        if params == "" {
            return self.dump_cauldron();
        }
        if let Some(pos) = self.satchel.iter().position(|x| x.container == Container::Bottle && x.matches_name(params)) {
            let mut bottled = self.satchel.remove(pos);
            self.empty_bottles += 1;
            return match bottled.solvent {
                Solvent::Vivo | Solvent:: Air => {
                    bottled.container = Container::None;
                    let result = format!("Unbottled {}.", bottled.full_name());
                    self.satchel.push(bottled);
                    result
                }
                _ => format!("Discarded {} and washed bottle.", bottled.full_name())
            }
        }
        if let Some(pos) = self.infusion_shelf.iter().position(|x| x.container == Container::Bottle && x.full_name() == params) {
            let bottled = self.infusion_shelf.remove(pos);
            self.empty_bottles += 1;
            return format!("Discarded infusing {}.", bottled.full_name())
        }
        return "You can't find anything to dump.".to_string();
    }

    fn stir(&mut self) -> String {
        if self.has_cauldron() { match &mut self.cauldron {
            Some(ingredient) => format!("{0}\n{1}", ingredient.boil(), ingredient.show_in_progress()),
            None => "The cauldron is empty.".to_string(),
        }} else {
            "You see nothing to stir.".to_string()
        }
    }

    fn fill_cauldron(&mut self, ingredient: &Ingredient) -> String {
        if !self.has_cauldron() {
            return "You don't have the equipment to brew potions out here.".to_string();
        }
        match &self.cauldron {
            Some(_) => "Specify an ingredient.".to_string(),
            None => {
                let mut ingredient = ingredient.clone();
                ingredient.container = Container::None;
                ingredient.update_effect();
                let descr = ingredient.show_in_progress();
                let name = ingredient.full_name();
                self.cauldron = Some(ingredient);
                format!("You pour {0} into the cauldron and bring it to a boil.\n{1}", name, descr)
            }
        }
    }

    fn decoct(&mut self, addition: Ingredient) -> String {
        if !self.has_cauldron() {
            return "You don't have the equipment to brew potions out here.".to_string();
        }
        if addition.container == Container::Bottle {
            self.empty_bottles += 1;
        }
        let mut added: Option<String> = None;
        if self.cauldron.is_none() {
            if let Solvent::Water | Solvent::Ether | Solvent::Oil = addition.solvent {
                return self.fill_cauldron(&addition);
            }
            added = Some(self.fill_cauldron(&WATER));
        }
        let decocted = self.cauldron.as_mut().unwrap().decoct(&addition);
        match added {
            Some(added) => format!("{}\n{}", added, decocted),
            None => decocted,
        }
    }

    fn infuse_named(&mut self, params: &str) -> String {
        let base = match self.take_ingredient(params, |_| true) {
            Ok(ingredient) => ingredient,
            Err(result) => return result,
        };
        let remainder = match base.search_remainder(params) {
            Some(needle) => needle,
            None => return self.infuse(WATER.clone(), base)
        };
        let addition = match self.take_ingredient(remainder, |_| true) {
            Ok(ingredient) => ingredient,
            Err(result) => return result,
        };
        self.infuse(base, addition)
    }

    fn infuse(&mut self, base: Ingredient, addition: Ingredient) -> String {
        match base.solvent {
            Solvent::Vivo | Solvent::Air => {
                let name = base.full_name();
                self.satchel.push(base);
                self.satchel.push(addition);
                return format!("The base for the infusion must be a liquid, not {}.", name)
            },
            Solvent::Water | Solvent::Ether | Solvent::Oil => (),
        }
        if let Container::Bottle = addition.container {
            self.empty_bottles += 1;
        }
        let mut base = base;
        match base.container {
            Container::Bottle => (),
            Container::None => match self.bottle(&mut base) {
                Ok(_result) => (),
                Err(result) => return result,
            },
        }
        let result = base.infuse(&addition);
        self.infusion_shelf.push(base);
        format!("Bottle of [{}] added to shelf to infuse over time.", result)
    }

    fn buy(&mut self, params: &str) -> String {
        if self.current_region != RegionEnum::Village {
            return "There's no one here to buy from.".to_string()
        }
        let bottle_price = 1;
        let spirits_price = 8;
        let oil_price = 24;
        match params {
            "bottle"|"b"|"" => {
                if self.money < bottle_price {
                    return format!("You only have {} silver and can't afford {} for a bottle", self.money, bottle_price);
                }
                self.money -= bottle_price;
                self.empty_bottles += 1;
                "You bought an empty bottle.".to_string()
            }
            "spirits" => {
                if self.money < spirits_price {
                    return format!("You only have {} silver and can't afford {} for some spirits", self.money, spirits_price);
                }
                self.money -= spirits_price;
                self.satchel.push(ETHER.clone());
                "You bought spirits.".to_string()
            }
            "oil" => {
                if self.money < oil_price {
                    return format!("You only have {} silver and can't afford {} for oil", self.money, oil_price);
                }
                self.money -= oil_price;
                self.satchel.push(OIL.clone());
                "You bought oil.".to_string()
            }
            _ => format!("The village doesn't sell '{}'", params)
        }
    }

    fn sell(&mut self, params: &str) -> String {
        if self.current_region != RegionEnum::Village {
            return "There's no one here to sell to.".to_string()
        }
        if params == "bottle" {
            if self.empty_bottles <= 0 {
                return "You have no bottles to sell".to_string();
            }
            let value = Container::Bottle.sale_value();
            self.empty_bottles -= 1;
            self.money += value;
            return format!("Sold a bottle for {} silver", value);
        }
        let item = match self.take_ingredient(params, |_| true) {
            Ok(ingredient) => ingredient,
            Err(result) => return result,
        };
        let value = item.sale_value();
        self.money += value;
        match item.container {
            Container::Bottle => { self.bottles_sold += 1; }
            Container::None => (),
        }
        format!("Sold {} for {} silver pieces", item.full_name(), value)
    }

    fn advance_time(&mut self) -> String {
        // Ingredients dry out or rot
        let herb_changes = self.satchel.iter_mut().filter_map(|i| i.advance_time()).collect::<Vec<String>>().join("\n");
        // Infusions complete
        let infused = self.infusion_shelf.len();
        self.satchel.append(&mut self.infusion_shelf);
        // Customers return bottles
        let prev_bottles = self.empty_bottles;
        for _i in 0..self.bottles_sold {
            if rand::random_bool(0.5) {
                self.empty_bottles += 1;
                self.bottles_sold -= 1;
            } else if rand::random_bool(0.1) {
                self.bottles_sold -= 1;
            }
        }
        let bottles_returned = self.empty_bottles - prev_bottles;
        // Herbs regrow
        for region in self.regions.values_mut() {
            region.regrow();
        }
        match (herb_changes.as_str(), infused > 0, bottles_returned > 0) {
            ("", true, false) => format!("Completed {} infusions.", infused),
            (_, true, false) => format!("{0}\nCompleted {1} infusions.", herb_changes, infused),
            ("", false, false) => "You wake refreshed.".to_string(),
            (_, false, false) => herb_changes,
            ("", true, true) => format!("Completed {} infusions.\nCustomers returned {} empty bottles.", infused, bottles_returned),
            (_, true, true) => format!("{0}\nCompleted {1} infusions.\nCustomers returned {2} empty bottles.", herb_changes, infused, bottles_returned),
            ("", false, true) => format!("Customers returned {} empty bottles.", bottles_returned),
            (_, false, true) => format!("{}\nCustomers returned {} empty bottles.", herb_changes, bottles_returned),
        }
    }

    fn look(&mut self) -> String {
        let region = &self.regions[self.current_region];
        format!("{}\n{}", region.name, region.description)
    }
}


fn help() -> String {
"==Navigation==
north, south, east, west, [location name] - travel
look - describe your current location
map - display a map of the area

==Foraging==
gather or forage - search for herbs in your current region, with priority for a specific herb if you name one
inv or satchel - list items inside your satchel

==Brewing==
book - read your alchemy instruction manual
brew [ingredient] - add the ingredient to the cauldron for a decoction
soak [ingredient] - add the ingredient to a bottle for an infusion
stir - stir the cauldron as it boils, allowing lighter elements to evaporate
bottle [ingredient] - put the named ingredient into a bottle, or finish and bottle what's brewing in the cauldron
dump - empty out the cauldron and get rid of the contents

==Misc==
sleep - advances time, allowing herbs to regrow, infusions to infuse, and fresh herbs to dry out
sell [item] - exchange goods for money at the village market
buy [item] - same deal, but money for goods
help - print this info".to_string()
}

#[wasm_bindgen]
pub fn welcome() -> String {
    "The sun shines through the aged hut's shutters as you wake up. You begin to roll over, then remember what day it is. Today is the day you're opening your very own alchemy shop!\nYou remember your trip in yesterday from the north, passing through the village. A local elder suggested you start looking for herbs in the forest to the south of your hut.\nType 'help' to list commands.".to_string()
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
            "wait"|"advance"|"sleep" => world.advance_time(),
            "inv"|"inventory"|"satchel" => world.list_inventory(),
            "gather"|"forage"|"collect" => world.forage(&params),
            "brew"|"decoct"|"cauldron" => {
                if params == "" {
                    world.fill_cauldron(&WATER)
                } else {
                    match world.take_ingredient(&params, |_| true) {
                        Ok(ingr) => world.decoct(ingr),
                        Err(e) => e,
                    }
                }
            }
            "soak"|"infuse" => world.infuse_named(&params),
            "bottle" => world.bottle_named(&params),
            "dump"|"spill"|"empty" => world.dump(&params),
            "stir" => world.stir(),
            "sell" => world.sell(&params),
            "buy" => world.buy(&params),
            "map" | "surroundings" => MAP.to_string(),
            "book"|"textbook"|"alchemy" => ALCHEMY_BOOK.to_string(),
            "look" => world.look(),
            "help" => help(),
            _ => format!("You're not sure how to '{}'. Try 'help'.", verb),
        }
    }
}
