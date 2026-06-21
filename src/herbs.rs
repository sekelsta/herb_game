use enum_map::EnumMap;
use once_cell::sync::Lazy;

use crate::{Container, Element, Ingredient, Modifier, Solvent};
use crate::Element::*;
use crate::Modifier::*;

pub static DANDELION: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Air][Provide] = 2;
    elements[Fire][Provide] = 1;
    elements[Water][Stabilize] = -1;
    Ingredient { name: "dandelion", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static VIOLET: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Shadow][Provide] = 2;
    elements[Shadow][Strengthen] = 1;
    Ingredient { name: "violet", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BLUEBELL: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Mana][Provide] = 2; // glue, toxin
    Ingredient { name: "bluebell", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WHITE_CLOVER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "white clover", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static RED_CLOVER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "red clover", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WATERMINT: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Provide] = 3;
    elements[Shadow][Provide] = 2;
    Ingredient { name: "watermint", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WINTERGREEN: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Provide] = 4;
    Ingredient { name: "wintergreen", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static HORSETAIL: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "horsetail", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static HEALALL: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "heal-all", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static JEWELWEED: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Provide] = 1;
    Ingredient { name: "jewelweed", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WILLOW: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "willow", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static COLTSFOOT: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "coltsfoot", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static MARSH_MALLOW: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "marsh mallow", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static YARROW: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "yarrow", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static SWEET_ANNIE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "sweet Annie", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BULL_THISTLE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "bull thistle", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BUTTERCUP: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "buttercup", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static SKUNK_CABBAGE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "skunk cabbage", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static MEADOWSWEET: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "meadowsweet", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static NEW_YORK_FERN: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "candlefern", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static PETTY_SPURGE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Strengthen] = 4;
    Ingredient { name: "petty spurge", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static VELVETLEAF: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "velvetleaf", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static PURSLANE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Provide] = 1;
    Ingredient { name: "purslane", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static JACK_IN_THE_PULPIT: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "Jack in the pulpit", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static TROUT_LILY: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "trout lily", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WILD_STRAWBERRY: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "wild strawberry", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static DAFFODIL: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "daffodil", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static COLUMBINE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "columbine", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WHITE_TRILLIUM: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "white trillium", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static LADY_FERN: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "lady fern", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static CINNAMON_FERN: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "cinnamon fern", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static FOX_SEDGE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "fox sedge", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static MEADOW_ANEMONE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "meadow anemone", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static MILKWEED: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Provide] = 1;
    Ingredient { name: "milkweed", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static JOE_PYE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "Joe-Pye weed", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static TURTLEHEAD: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "turtlehead", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static SPOTTED_DEADNETTLE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "spotted deadnettle", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static OXEYE_DAISY: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "ox-eye daisy", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static ENCHANTERS_NIGHTSHADE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "enchanter's nightshade", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static POISON_HEMLOCK: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "poison hemlock", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static YEW: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "yew berries", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BLACK_NIGHTSHADE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "black nightshade", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static DEADLY_NIGHTSHADE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "deadly nightshade", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BITTERWEET: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "bittersweet nightshade", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static PASTURE_ROSE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "pasture rose", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BORAGE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "borage", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static FEVERFEW: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Ice][Provide] = 1;
    Ingredient { name: "feverfew", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static CHAMOMILE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "chamomile", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static FLEABANE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "fleabane", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BURDOCK: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "burdock", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static YELLOW_DOCK: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "bitter dock", solvent: Solvent::Vivo, container: Container::None, elements }
});
