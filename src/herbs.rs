use enum_map::EnumMap;
use once_cell::sync::Lazy;

use crate::{Container, Element, Ingredient, IngredientKind, Modifier, RegionEnum, Solvent};
use crate::Element::*;
use crate::Modifier::*;
use crate::RegionEnum::*;

pub struct Herb {
    pub name: &'static str,
    pub tier: i32,
    //pub sun: [f32; 2],
    //pub moisture: [f32; 2],
    pub toxicity: f32,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
    pub biomes: Vec<RegionEnum>,
}

impl Herb {
    pub fn to_ingredient(&self) -> Ingredient {
        Ingredient {
            kind: IngredientKind::Herb { name: self.name },
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

pub static REFERENCE_HERBS: Lazy<Vec<Herb>> = Lazy::new(|| vec!(
// ------ Beginner herbs ------ //
    // Elements: Earth, Water, Air, Fire
    // Accessible modifiers: Provide
    Herb {
        name: "dandelion",
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
    Herb {
        name: "white clover",
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Earth, Provide, 1),
            (Light, Stabilize, 1),
            (Shadow, Strengthen, -2),
            (Mana, Stabilize, 1),
        ]),
        biomes: vec!(Field, Village),
    },
    Herb {
        name: "red clover",
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Fire, Provide, 1),
            (Water, Provide, 1),
            (Earth, Provide, 1),
            (Earth, Stabilize, 2),
            (Light, Strengthen, 2),
        ]),
        biomes: vec!(Field, WildflowerMeadow),
    },
    Herb {
        name: "wild basil", // Clinopodium vulgare
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 1),
            (Fire, Provide, 1),
        ]),
        biomes: vec!(Garden, Field),
    },
    Herb {
        name: "yarrow",
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 3),
            (Shadow, Strengthen, 2),
            (Earth, Provide, 1),
        ]),
        biomes: vec!(Field, WildflowerMeadow),
    },
    Herb {
        name: "wild strawberry",
        tier: 0,
        toxicity: 0.0,
        elements: map_elements(&[
            (Air, Provide, 1),
            (Water, Provide, 1),
            (Ice, Strengthen, 1),
        ]),
        biomes: vec!(FriendlyForest),
    },
// ------ Tier 1 herbs ------ //
    // New elements: Spirit, Light, Shadow
    // Accessible modifiers: Provide, Strengthen
    Herb {
        name: "rose",
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
    Herb {
        name: "sunflower",
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Light, Provide, 3),
            (Earth, Provide, 1),
            (Fire, Strengthen, 2),
            (Ice, Strengthen, -3),
        ]),
        biomes: vec!(Field, Village, WildflowerMeadow),
    },
// ------ Tier 2 herbs ------ //
    // New elements: Ice, Thunder, Mana
    // Accessible modifiers: Provide, Strengthen, Stabilize
    Herb {
        name: "watermint",
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Ice, Provide, 3),
            (Air, Strengthen, 3),
            (Shadow, Provide, 2),
            (Water, Provide, 1),
        ]),
        biomes: vec!(MeadowRiver),
    },
    Herb {
        name: "heal-all",
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
    Herb {
        name: "burdock",
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
    Herb {
        name: "bull thistle",
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
// ------ Tier 3 herbs ------ //
    Herb {
        name: "jack-in-the-pulpit",
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
    Herb {
        name: "bluebell",
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
            (Mana, Provide, 2),
            (Void, Provide, 2),
            (Taint, Provide, 1),
        ]),
        biomes: vec!(FriendlyForest),
    },
    Herb {
        name: "tulip",
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
    Herb {
        name: "jewelweed",
        tier: 3,
        toxicity: 0.1,
        elements: map_elements(&[
            (Ice, Provide, 2),
            (Mana, Provide, 2),
            (Air, Stabilize, 3),
            (Thunder, Stabilize, -1),
        ]),
        biomes: vec!(ForestRiver),
    },
    Herb {
        name: "ox-eye daisy",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Light, Provide, 1),
            (Air, Provide, 1),
            (Light, Stabilize, 3),
            (Shadow, Stabilize, -3)
        ]),
        biomes: vec!(WildflowerMeadow),
    },
    Herb {
        name: "skunk cabbage",
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
    Herb {
        name: "wintergreen",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Ice, Provide, 4),
            (Earth, Provide, 2),
            (Earth, Stabilize, 2),
        ]),
        biomes: vec!(PineForest),
    },
    Herb {
        name: "fox sedge",
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
    Herb {
        name: "spotted deadnettle",
        tier: 3,
        toxicity: 0.05,
        elements: map_elements(&[
            (Shadow, Provide, 3),
            (Earth, Provide, 2),
            (Void, Stabilize, 2),
        ]),
        biomes: vec!(ForestRiver),
    },
));

pub static TODO_HERBS: Lazy<Vec<Herb>> = Lazy::new(|| vec!(
// ------ Tier 1 herbs ------ //
    Herb {
        name: "daffodil",
        tier: 1,
        toxicity: 0.3,
        elements: map_elements(&[
            (Light, Provide, 3),
        ]),
        biomes: vec!(Garden, Village, FriendlyForest),
    },
    Herb {
        name: "lawn daisy",
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(Village),
    },
    Herb {
        name: "violet",
        tier: 1,
        toxicity: 0.0,
        elements: map_elements(&[
            (Shadow, Provide, 2),
            (Shadow, Strengthen, 2),
        ]),
        biomes: vec!(FriendlyForest, PineForest),
    },
    Herb {
        name: "buttercup",
        tier: 1,
        toxicity: 0.2,
        elements: map_elements(&[
            (Fire, Provide, 1),
            (Light, Strengthen, 2),
        ]),
        biomes: vec!(WildflowerMeadow),
    },
// ------ Tier 2 herbs ------ //
    Herb {
        name: "watercress",
        tier: 2,
        toxicity: 0.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(MeadowRiver),
    },
    Herb {
        name: "yew",
        tier: 2,
        toxicity: 1.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(),
    },
    Herb {
        name: "horsetail",
        tier: 2,
        toxicity: 0.01,
        elements: map_elements(&[
            (Thunder, Provide, 1),
            (Thunder, Strengthen, 3),
        ]),
        biomes: vec!(Village, MeadowRiver),
    },
    Herb {
        name: "willow",
        tier: 2,
        toxicity: 0.1,
        elements: map_elements(&[
        ]),
        biomes: vec!(MeadowRiver),
    },
// ------ Tier 3 herbs + unsorted ------ //
    Herb {
        name: "way-broad", // Broadleaf plantain
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(Garden, Village),
    },
    Herb {
        name: "feverfew",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Ice, Provide, 1),
        ]),
        biomes: vec!(Garden),
    },
    Herb {
        name: "candlefern", // New York fern
        tier: 3,
        toxicity: 0.1,
        elements: map_elements(&[
        ]),
        biomes: vec!(Garden, FriendlyForest),
    },
    Herb {
        name: "trout lily",
        tier: 3,
        toxicity: 0.2,
        elements: map_elements(&[
        ]),
        biomes: vec!(FriendlyForest),
    },
    Herb {
        name: "enchanter's nightshade",
        tier: 3,
        toxicity: 0.05,
        elements: map_elements(&[
            (Thunder, Stabilize, 4),
        ]),
        biomes: vec!(Garden),
    },
    Herb {
        name: "fleabane",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(Garden),
    },
    Herb {
        name: "bittersweet nightshade",
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
        ]),
        biomes: vec!(FriendlyForest),
    },
    Herb {
        name: "black nightshade",
        tier: 3,
        toxicity: 0.05,
        elements: map_elements(&[
        ]),
        biomes: vec!(Village, Field),
    },
    Herb {
        name: "deadly nightshade",
        tier: 3,
        toxicity: 1.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(PineForest),
    },
    Herb {
        name: "petty spurge",
        tier: 3,
        toxicity: 0.3,
        elements: map_elements(&[
            (Shadow, Provide, 1),
            (Taint, Provide, 1),
            (Ice, Strengthen, 4),
        ]),
        biomes: vec!(Village, Field),
    },
    Herb {
        name: "coltsfoot",
        tier: 3,
        toxicity: 0.01,
        elements: map_elements(&[
            (Thunder, Provide, 2),
        ]),
        biomes: vec!(ForestRiver, MeadowRiver),
    },
    Herb {
        name: "purslane",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
            (Water, Provide, 2),
            (Ice, Provide, 1),
        ]),
        biomes: vec!(Field),
    },
    Herb {
        name: "velvetleaf",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(Field),
    },
    Herb {
        name: "fleabane",
        tier: 3,
        toxicity: 0.0,
        elements: map_elements(&[
        ]),
        biomes: vec!(Field),
    },
    Herb {
        name: "milkweed",
        tier: 3,
        toxicity: 0.1,
        elements: map_elements(&[
            (Ice, Strengthen, 2),
        ]),
        biomes: vec!(WildflowerMeadow),
    },
    Herb {
        name: "columbine",
        tier: 3,
        toxicity: 0.1,
        elements: map_elements(&[
            (Thunder, Provide, 1),
        ]),
        biomes: vec!(PineForest, FriendlyForest),
    },
));
// PineForest: vec!(&*WHITE_TRILLIUM, &*LADY_FERN, &*YEW),
// WildflowerMeadow: vec!(&*MILKWEED, &*SWEET_ANNIE, &*POISON_HEMLOCK, &*PASTURE_ROSE, &*CHAMOMILE, &*BORAGE, &*YELLOW_DOCK),
// MeadowRiver: vec!(&*TURTLEHEAD, &*JOE_PYE, &*MEADOW_ANEMONE, &*WILLOW, &*MARSH_MALLOW, &*COMFREY, &8BONESET),
// ForestRiver: vec!(&*CINNAMON_FERN, &*MEADOWSWEET),


// Plus sow thistle, chickweed, field mustard, poor man's pepper, lamb's quarters, wintercress, greenbriar, carrion flower, st john's wort, evening primrose, ragwort, ragweed, bouncing bet/soapwort

