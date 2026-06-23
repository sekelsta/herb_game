use enum_map::{enum_map, Enum, EnumMap};
use rand::seq::SliceRandom;
use strum_macros::EnumString;

use crate::KnowledgeState;
use crate::herbs::*;

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
PPDPPDDPD | |  __ | |    wwwwwwwwww/
PDPPDPDDDP              wwwwwwwwww//
PDPDDDDDDD       D _   DwwDwwwDw//ww
DDDDPDDD  |~~~~|  / \   wwwwDDw//www
DDDDDDDDD |~~~~|  |H| DDwDDwD//DDwDw
DDDDDDDDDDDD      DDDDDDDD/~/wDDwDDw
DDDDDDDDDDDDDDDDDDDDDDD//D//DDDDDDDw
DDDDDDDDDDDDDDDDDDDD//DDD//DDDDDDDDD
DDDDDDDDDDDDDDDD//DDDDDD//DDDDDDDDDD
DDDDDDDDDDDDDD//DDDDDDD||DDDDDDDDDDD
DDDDDDDDDDDD//DDDDDDDDDD\\DDDDDDDDDD
* = You are here"#;

fn map_with_star(x: i32, y: i32) -> String {
    let header = "Surroundings:\n".len() as i32;
    let line_len = 37;
    let loc = (header + y * line_len + x) as usize;
    let mut result = MAP.to_string();
    // ASCII characters only
    unsafe {
        let bytes: &mut [u8] = result.as_bytes_mut();
        bytes[loc] = '*' as u8;
    }
    result
}

#[derive(Clone, Copy, Debug, Enum, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Direction {
    #[strum(serialize = "n", serialize = "north")]
    North,
    #[strum(serialize = "s", serialize = "south")]
    South,
    #[strum(serialize = "e", serialize = "east")]
    East,
    #[strum(serialize = "w", serialize = "west")]
    West,
    #[strum(serialize = "ne", serialize = "northeast")]
    Northeast,
    #[strum(serialize = "nw", serialize = "northwest")]
    Northwest,
    #[strum(serialize = "se", serialize = "southeast")]
    Southeast,
    #[strum(serialize = "sw", serialize = "southwest")]
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
    pub name: &'static str,
    pub description: &'static str,
    pub routes: EnumMap<Direction, RegionEnum>,
    pub current_herbs: Vec<&'static Herb>,
    x: i32, y: i32,
}

impl Region {
    pub fn new_regions() -> EnumMap<RegionEnum, Self> {
        use Direction::*;
        use RegionEnum::*;
        enum_map!(
            Hut => Region {
                name: "Home Sweet Home",
                description: "A simple hut with a cauldron and rack of drying herbs in the back.",
                routes: enum_map!(
                    West => Garden,
                    East | Northeast => MeadowRiver,
                    South | Southeast => ForestRiver,
                    Southwest => FriendlyForest,
                    North | Northwest => Village,
                ),
                current_herbs: Vec::new(),
                x: 19, y: 13,
            },
            Garden => Region {
                name: "Your Garden",
                description: "Needs some work. You decide you'd rather keep foraging from the wild.\nA few herbs have survived from better-tended times, with occasional weeds sprouting around them.",
                routes: enum_map!(
                    East => Hut,
                    Northwest => PineForest,
                    South | Southeast | Southwest | West => FriendlyForest,
                    North | Northeast => Village,
                ),
                current_herbs: Vec::new(),
                x: 13, y: 13,
            },
            FriendlyForest => Region {
                name: "Friendly Forest",
                description: "Dapples of light filter though the soft leaves.",
                routes: enum_map!(
                    North | Northwest => Garden,
                    East | Southeast => ForestRiver,
                    Northeast => Hut,
                    West | South | Southwest => FriendlyForest,
                ),
                current_herbs: Vec::new(),
                x: 8, y: 16,
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
                x: 15, y: 9,
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
                x: 17, y: 3,
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
                x: 4, y: 3,
            },
            WildflowerMeadow => Region {
                name: "Wildflower Meadow",
                description: "Tall grass for haying, interrupted by colorful flowers.",
                routes: enum_map!(
                    West | Southwest => Field,
                    South | Southeast => MeadowRiver,
                    _ => WildflowerMeadow,
                ),
                current_herbs: Vec::new(),
                x: 30, y: 3,
            },
            MeadowRiver => Region {
                name: "Meadow Riverbank",
                description: "A river flows beside the meadow, the bright sun sparkling off its waters.",
                routes: enum_map!(
                    North => WildflowerMeadow,
                    Northwest => Village,
                    West => Hut,
                    South | Southwest => ForestRiver,
                    _ => MeadowRiver,
                ),
                current_herbs: Vec::new(),
                x: 31, y: 11,
            },
            ForestRiver => Region {
                name: "Forest Riverbank",
                description: "A river flows through the forest.",
                routes: enum_map!(
                    North => Hut,
                    Northwest => Garden,
                    West => FriendlyForest,
                    Northeast | East => MeadowRiver,
                    _ => ForestRiver,
                ),
                current_herbs: Vec::new(),
                x: 19, y: 15,
            },
        )
    }

    pub fn local_map(&self) -> String {
        map_with_star(self.x, self.y)
    }

    pub fn regrow(&mut self, biome: &RegionEnum) {
        for i in (0..self.current_herbs.len()).rev() {
            if rand::random_bool(0.5) {
                self.current_herbs.remove(i);
            }
        }
        for herb in REFERENCE_HERBS.iter() {
            if herb.biomes.contains(biome) {
                let chance = (5 - herb.tier) as f64 / 15.0; // Written when highest herb tier is 3
                if rand::random_bool(chance) {
                    self.current_herbs.push(herb);
                }
                if rand::random_bool(chance) {
                    self.current_herbs.push(herb);
                    self.current_herbs.push(herb);
                }
            }
        }
        self.current_herbs.shuffle(&mut rand::rng());
    }

    pub fn status(&self, discoveries: &KnowledgeState) -> String {
        if self.current_herbs.iter().any(|h| h.tier <= discoveries.herb_tier) {
            return "You spot some herbs you recognize.".to_string();
        }
        "".to_string()
    }
}
