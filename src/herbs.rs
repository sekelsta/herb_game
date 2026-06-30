use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use strum_macros::IntoStaticStr;

use crate::{Container, Element, Ingredient, IngredientKind, Modifier, RegionEnum, Solvent};
use crate::Element::*;
use crate::Modifier::*;
use crate::RegionEnum::*;

use Plant::*;

#[derive(Clone, Copy, Debug, IntoStaticStr, Enum, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Plant {
    Dandelion,
    WhiteClover,
    RedClover,
    WildBasil, // Clinopodium vulgare
    Yarrow,
    WildStrawberry,
    Watercress,
    Waybroad, // Broadleaf plantain
    Rose,
    Sunflower,
    OxeyeDaisy,
    Daffodil,
    LawnDaisy,
    Violet,
    Buttercup,
    Candlefern, // New York fern
    Watermint,
    Healall,
    Horsetail,
    Burdock,
    BullThistle,
    Jewelweed,
    Wintergreen,
    Yew,
    Willow,
    Ferverfew,
    TroutLily,
    Coltsfoot,
    FoxSedge,
    SpottedDeadnettle,
    JackInThePulpit,
    Bluebell,
    Tulip,
    SkunkCabbage,
    EnchantersNightshade,
    BittersweetNightshade,
    BlackNightshade,
    DeadlyNightshade,
    PettySpurge,
    Purslane,
    Velvetleaf,
    Milkweed,
    Columbine,
}

impl Plant {
    pub fn to_static_str(&self) -> &'static str {
        match self {
            WhiteClover => "white clover",
            RedClover => "red clover",
            WildBasil => "wild basil",
            WildStrawberry => "wild strawberry",
            Waybroad => "way-broad",
            OxeyeDaisy => "ox-eye daisy",
            LawnDaisy => "lawn daisy",
            Healall => "heal-all",
            BullThistle => "bull thistle",
            TroutLily => "trout lily",
            FoxSedge => "fox sedge",
            SpottedDeadnettle => "spotted deadnettle",
            JackInThePulpit => "Jack-in-the-pulpit",
            SkunkCabbage => "skunk cabbage",
            EnchantersNightshade => "enchanter's nightshade",
            BittersweetNightshade => "bittersweet nightshade",
            BlackNightshade => "black nightshade",
            DeadlyNightshade => "deadly nightshade",
            PettySpurge => "petty spurge",
            e => e.into(),
        }
    }

    pub fn to_lowercase_string(&self) -> String {
        match self {
            // Keep Jack capitalized
            JackInThePulpit => self.to_static_str().to_string(),
            _ => self.to_static_str().to_ascii_lowercase(),
        }
    }

    pub fn to_ingredient(&self) -> Ingredient {
        REFERENCE_HERBS[*self].to_ingredient(*self)
    }
}

pub struct Herb {
    pub tier: i32,
    //pub sun: [f32; 2],
    //pub moisture: [f32; 2],
    pub toxicity: f32,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
    pub biomes: Vec<RegionEnum>,
}

impl Herb {
    pub fn to_ingredient(&self, species: Plant) -> Ingredient {
        Ingredient {
            kind: IngredientKind::Herb { species },
            solvent: Solvent::Vivo,
            container: Container::None,
            elements: self.elements.clone(),
            toxicity: self.toxicity,
            effect: None,
            strength: 0.0,
            is_tainted: false,
        }
        // Deliberately do not check for potion effects - you need to do something with the herb first
    }
}

fn map_elements(entries: &[(Element, Modifier, i32)]) -> EnumMap<Element, EnumMap<Modifier, i32>> {
    let mut e: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    for &(element, modifier, amount) in entries {
        e[element][modifier] = amount;
    }
    e
}

pub static REFERENCE_HERBS: Lazy<EnumMap<Plant, Herb>> = Lazy::new(|| enum_map!(
// ------ Beginner herbs ------ //
    // Elements: Earth, Water, Air, Fire
    // Accessible modifiers: Provide
    Dandelion => Herb {
        tier: 0,
        //sun: [0.5, 1.0],
        //moisture: [0.2, 0.8],
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 2),
            (Fire, Provide, 1),
            (Earth, Provide, 2),
            (Spirit, Strengthen, 2),
        ]),
        biomes: vec!(Field, Garden, Village),
    },
    WhiteClover => Herb {
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Earth, Provide, 1),
            (Light, Stabilize, 1),
            (Shadow, Strengthen, -2),
            (Mana, Strengthen, 1),
        ]),
        biomes: vec!(Field, Village),
    },
    RedClover => Herb {
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Fire, Provide, 1),
            (Water, Provide, 1),
            (Earth, Stabilize, 2),
            (Light, Strengthen, 2),
        ]),
        biomes: vec!(Field, WildflowerMeadow),
    },
    WildBasil => Herb {
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 1),
            (Fire, Provide, 1),
        ]),
        biomes: vec!(Garden, Field),
    },
    Yarrow => Herb {
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 3),
            (Shadow, Strengthen, 2),
            (Earth, Provide, 1),
        ]),
        biomes: vec!(WildflowerMeadow),
    },
    WildStrawberry => Herb {
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 1),
            (Water, Provide, 1),
            (Ice, Strengthen, 1),
        ]),
        biomes: vec!(FriendlyForest),
    },
    Watercress => Herb {
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Water, Provide, 2),
            (Water, Stabilize, 1),
            (Water, Strengthen, 1),
            (Fire, Provide, 1),
            (Shadow, Stabilize, 1),
        ]),
        biomes: vec!(MeadowRiver),
    },
// ------ Tier 1 herbs ------ //
    // New elements: Spirit, Light, Shadow
    // Accessible modifiers: Provide, Strengthen, Stabilize
    Waybroad => Herb {
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Earth, Provide, 1),
            (Air, Stabilize, 1),
            (Light, Stabilize, 1),
        ]),
        biomes: vec!(Village),
    },
    Rose => Herb {
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 1),
            (Light, Provide, 1),
            (Light, Strengthen, 1),
            (Earth, Provide, 1),
            (Spirit, Strengthen, 1),
            (Taint, Strengthen, -2),
        ]),
        biomes: vec!(Garden),
    },
    Sunflower => Herb {
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Light, Provide, 3),
            (Earth, Provide, 1),
            (Fire, Strengthen, 2),
            (Ice, Strengthen, -3),
        ]),
        biomes: vec!(Field, WildflowerMeadow),
    },
    OxeyeDaisy => Herb {
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Light, Provide, 1),
            (Air, Provide, 1),
            (Light, Stabilize, 3),
            (Shadow, Stabilize, -2)
        ]),
        biomes: vec!(WildflowerMeadow),
    },
    Daffodil => Herb {
        tier: 1,
        toxicity: 0.3,
        elements: map_elements(&[
            (Light, Provide, 2),
            (Earth, Provide, 1),
        ]),
        biomes: vec!(Garden, FriendlyForest, PineForest),
    },
    LawnDaisy => Herb {
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Earth, Strengthen, 1),
            (Mana, Strengthen, 1),
            (Light, Stabilize, 1),
            (Fire, Strengthen, -1),
        ]),
        biomes: vec!(Village),
    },
    Violet => Herb {
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Shadow, Provide, 2),
            (Shadow, Strengthen, 2),
            (Water, Provide, 1),
            (Light, Stabilize, 1),
        ]),
        biomes: vec!(FriendlyForest, PineForest),
    },
    Buttercup => Herb {
        tier: 1,
        toxicity: 0.2,
        elements: map_elements(&[
            (Fire, Provide, 2),
            (Water, Strengthen, -1),
            (Light, Strengthen, 2),
            (Air, Stabilize, -1),
        ]),
        biomes: vec!(WildflowerMeadow),
    },
    Candlefern => Herb { // New York fern
        tier: 1,
        toxicity: 0.1,
        elements: map_elements(&[
            (Light, Provide, 1),
            (Air, Provide, 1),
            (Fire, Strengthen, 1),
            (Earth, Provide, 1),
            (Mana, Stabilize, 1),
        ]),
        biomes: vec!(FriendlyForest),
    },
// ------ Tier 2 herbs ------ //
    // New elements: Ice, Thunder, Mana
    Watermint => Herb {
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
            (Ice, Provide, 3),
            (Air, Strengthen, 3),
            (Shadow, Provide, 2),
            (Water, Provide, 1),
        ]),
        biomes: vec!(MeadowRiver),
    },
    Healall => Herb {
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
            (Earth, Provide, 2),
            (Water, Provide, 1),
            (Thunder, Strengthen, 1),
            (Light, Strengthen, 3),
            (Shadow, Provide, 1),
            (Shadow, Stabilize, 1),
        ]),
        biomes: vec!(Village),
    },
    Horsetail => Herb {
        tier: 2,
        toxicity: 0.01,
        elements: map_elements(&[
            (Earth, Provide, 1),
            (Thunder, Provide, 1),
            (Thunder, Strengthen, 2),
        ]),
        biomes: vec!(Village, MeadowRiver),
    },
    Burdock => Herb {
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
            (Earth, Provide, 3),
            (Mana, Provide, 2),
            (Thunder, Provide, 1),
            (Fire, Stabilize, 3),
            (Ice, Stabilize, -3),
        ]),
        biomes: vec!(FriendlyForest, WildflowerMeadow, Field),
    },
    BullThistle => Herb {
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
            (Thunder, Provide, 2),
            (Thunder, Strengthen, 2),
            (Thunder, Stabilize, -1),
            (Fire, Provide, 1),
            (Fire, Strengthen, 3),
            (Shadow, Stabilize, -2),
            (Mana, Stabilize, -2),
        ]),
        biomes: vec!(MeadowRiver, WildflowerMeadow),
    },
    Jewelweed => Herb {
        tier: 2,
        toxicity: 0.1,
        elements: map_elements(&[
            (Ice, Provide, 2),
            (Mana, Provide, 2),
            (Air, Stabilize, 3),
            (Thunder, Stabilize, -1),
        ]),
        biomes: vec!(ForestRiver),
    },
    Wintergreen => Herb {
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
            (Ice, Provide, 4),
            (Earth, Provide, 2),
            (Earth, Stabilize, 2),
        ]),
        biomes: vec!(PineForest),
    },
    Yew => Herb {
        tier: 2,
        toxicity: 1.0,
        elements: map_elements(&[
            (Shadow, Provide, 2),
            (Fire, Provide, 1),
            (Ice, Strengthen, 1),
            (Water, Stabilize, -1),
            (Mana, Strengthen, 1),
        ]),
        biomes: vec!(PineForest),
    },
    Willow => Herb {
        tier: 2,
        toxicity: 0.1,
        elements: map_elements(&[
            (Earth, Provide, 1),
            (Water, Strengthen, 2),
            (Ice, Strengthen, 1),
            (Ice, Stabilize, 1),
        ]),
        biomes: vec!(MeadowRiver),
    },
    Ferverfew => Herb {
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
            (Ice, Provide, 1),
            (Air, Provide, 1),
            (Fire, Stabilize, 1),
            (Light, Stabilize, -1),
        ]),
        biomes: vec!(Garden),
    },
    TroutLily => Herb {
        tier: 2,
        toxicity: 0.2,
        elements: map_elements(&[
            (Shadow, Provide, 1),
            (Water, Provide, 1),
            (Light, Stabilize, 1),
            (Fire, Stabilize, -1),
            (Mana, Strengthen, 1),
        ]),
        biomes: vec!(FriendlyForest),
    },
    Coltsfoot => Herb {
        tier: 2,
        toxicity: 0.01,
        elements: map_elements(&[
            (Thunder, Provide, 2),
            (Earth, Provide, 1),
            (Ice, Stabilize, -1),
        ]),
        biomes: vec!(ForestRiver, MeadowRiver),
    },
// ------ Tier 3 herbs ------ //
    FoxSedge => Herb {
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Water, Strengthen, 3),
            (Shadow, Strengthen, 2),
            (Mana, Strengthen, -1),
            (Void, Stabilize, 3),
        ]),
        biomes: vec!(ForestRiver),
    },
    SpottedDeadnettle => Herb {
        tier: 3,
        toxicity: 0.05,
        elements: map_elements(&[
            (Shadow, Provide, 3),
            (Earth, Provide, 2),
            (Void, Stabilize, 2),
        ]),
        biomes: vec!(ForestRiver),
    },
    JackInThePulpit => Herb {
        tier: 3,
        toxicity: 0.6,
        elements: map_elements(&[
            (Mana, Provide, 3),
            (Shadow, Provide, 2),
            (Light, Strengthen, 2),
            (Ice, Stabilize, 3),
            (Earth, Stabilize, -2),
            (Taint, Stabilize, -3),
            (Spirit, Stabilize, 3),
        ]),
        biomes: vec!(FriendlyForest),
    },
    Bluebell => Herb {
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
            (Mana, Provide, 2),
            (Void, Provide, 2),
            (Taint, Provide, 1),
        ]),
        biomes: vec!(FriendlyForest),
    },
    Tulip => Herb {
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
            (Water, Provide, 1),
            (Ice, Provide, 2),
            (Void, Provide, 1),
            (Ice, Strengthen, 1),
            (Void, Stabilize, 3),
            (Spirit, Stabilize, -2),
        ]),
        biomes: vec!(Garden, Village),
    },
    SkunkCabbage => Herb {
        tier: 3,
        toxicity: 0.6,
        elements: map_elements(&[
            (Fire, Provide, 4),
            (Air, Provide, 3),
            (Air, Strengthen, 2),
            (Air, Stabilize, -3),
            (Shadow, Provide, 3),
            (Taint, Provide, 1),
            (Spirit, Provide, 1),
        ]),
        biomes: vec!(ForestRiver),
    },
    EnchantersNightshade => Herb {
        tier: 3,
        toxicity: 0.05,
        elements: map_elements(&[
            (Mana, Provide, 2),
            (Taint, Provide, 1),
            (Thunder, Stabilize, 4),
            (Air, Stabilize, 2),
            (Shadow, Stabilize, 3),
        ]),
        biomes: vec!(Garden, FriendlyForest),
    },
    // TODO: Herbs beyond this point have not had elements double-checked
    BittersweetNightshade => Herb {
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
            (Fire, Provide, 1),
            (Fire, Strengthen, 1),
            (Shadow, Provide, 1),
            (Shadow, Stabilize, -1),
        ]),
        biomes: vec!(FriendlyForest),
    },
    BlackNightshade => Herb {
        tier: 3,
        toxicity: 0.05,
        elements: map_elements(&[
            (Shadow, Provide, 2),
            (Earth, Strengthen, 1),
            (Fire, Stabilize, 1),
            (Ice, Stabilize, 1),
        ]),
        biomes: vec!(Village, Field),
    },
    DeadlyNightshade => Herb {
        tier: 3,
        toxicity: 1.0,
        elements: map_elements(&[
            (Shadow, Provide, 3),
            (Mana, Provide, 2),
            (Taint, Provide, 1),
            (Void, Strengthen, 1),
        ]),
        biomes: vec!(PineForest),
    },
    PettySpurge => Herb {
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
            (Shadow, Provide, 1),
            (Taint, Provide, 1),
            (Ice, Strengthen, 4),
        ]),
        biomes: vec!(Village, Field),
    },
    Purslane => Herb {
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Water, Provide, 2),
            (Ice, Provide, 1),
            (Earth, Stabilize, 1),
        ]),
        biomes: vec!(Field),
    },
    Velvetleaf => Herb {
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Mana, Provide, 1),
            (Earth, Strengthen, 1),
            (Water, Stabilize, 2),
        ]),
        biomes: vec!(Field),
    },
    Milkweed => Herb {
        tier: 3,
        toxicity: 0.1,
        elements: map_elements(&[
            (Light, Provide, 1),
            (Water, Provide, 1),
            (Earth, Provide, 1),
            (Ice, Strengthen, 2),
            (Fire, Stabilize, 1),
            (Mana, Stabilize, 1),
        ]),
        biomes: vec!(WildflowerMeadow),
    },
    Columbine => Herb {
        tier: 3,
        toxicity: 0.1,
        elements: map_elements(&[
            (Thunder, Provide, 1),
            (Mana, Provide, 1),
            (Void, Provide, 1)
        ]),
        biomes: vec!(PineForest, FriendlyForest),
    },
));
// PineForest: vec!(&*WHITE_TRILLIUM, &*LADY_FERN),
// WildflowerMeadow: vec!(&*MILKWEED, &*SWEET_ANNIE, &*POISON_HEMLOCK, &*PASTURE_ROSE, &*CHAMOMILE, &*BORAGE, &*YELLOW_DOCK),
// MeadowRiver: vec!(&*TURTLEHEAD, &*JOE_PYE, &*MEADOW_ANEMONE, &*MARSH_MALLOW, &*COMFREY, &8BONESET),
// ForestRiver: vec!(&*CINNAMON_FERN, &*MEADOWSWEET),


// Plus sow thistle, chickweed, field mustard, poor man's pepper, lamb's quarters, wintercress, greenbriar, carrion flower, st john's wort, evening primrose, ragwort, ragweed, bouncing bet/soapwort, fleabane

