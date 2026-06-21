use std::fmt;
use once_cell::sync::Lazy;

use enum_map::{Enum, EnumMap};

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

pub static WATER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Water][Modifier::Provide] = 2;
    Ingredient { name: "water", solvent: Solvent::Water, container: Container::None, elements }
});

pub static DANDELION: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Air][Modifier::Provide] = 2;
    elements[Element::Fire][Modifier::Provide] = 1;
    elements[Element::Water][Modifier::Stabilize] = -1;
    Ingredient { name: "dandelion", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static VIOLET: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "violet", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static BLUEBELL: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
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
    Ingredient { name: "watermint", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static WINTERGREEN: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
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
    Ingredient { name: "petty spurge", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static VELVETLEAF: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    Ingredient { name: "velvetleaf", solvent: Solvent::Vivo, container: Container::None, elements }
});

pub static PURSLANE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
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

#[derive(Clone, Copy, Debug, Enum, PartialEq)]
pub enum Element {
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
pub enum Modifier {
    Strengthen, // Weaken if value is negative
    Stabilize, // Destabilize if value is negative
    Provide,
    //Join,
    //Split,
}

#[derive(Clone, Copy, Debug)]
pub enum Solvent {
    Air,
    Water,
    Ether,
    Oil,
    Vivo, // Fresh herb
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Container {
    Bottle,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum IngredientKind {
    FreshHerb,
    DryHerb,
    Decoction, // Boil for herbal tea
    Infusion, // Cold soak
    Tincture, // Soak in alcohol
    Oil, // Or essence. Mild heat to speed up extraction, or cold to preserve aromatics
    Salve, // Add wax to the oil
    Poultice,
    Incense, // For resins, or Smudge for leaves/flowers
    Smudge,
    Ash,
    Salt,
}

impl fmt::Display for IngredientKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub name: &'static str,
    pub solvent: Solvent,
    pub container: Container,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
}

impl Ingredient {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn boil(&mut self) -> String {
        // One air evaporates, if present
        let air_evaporated = self.elements[Element::Air][Modifier::Provide] > 0;
        self.elements[Element::Air][Modifier::Provide] = (self.elements[Element::Air][Modifier::Provide] - 1).max(0);
        // Taint spreads
        let tainted = self.elements[Element::Taint][Modifier::Provide];
        let mut taint_spread = false;
        for _ in 0..tainted {
            let mut most = TAINTABLE_ELEMENTS[0];
            for e in TAINTABLE_ELEMENTS {
                if self.elements[e][Modifier::Provide] > self.elements[most][Modifier::Provide] {
                    most = e;
                }
            }
            if self.elements[most][Modifier::Provide] > 0 {
                self.elements[most][Modifier::Provide] -= 1;
                self.elements[Element::Taint][Modifier::Provide] += 1;
                taint_spread = true;
            }
        }
        match (taint_spread, air_evaporated) {
            (false, false) => "The cauldron boils.".to_string(),
            (false, true) => "The cauldron boils. Elemental air evaporates.".to_string(),
            (true, false) => "The cauldron boils. Taint spreads.".to_string(),
            (true, true) => "The cauldron boils. Elemental air evaporates. Taint spreads.".to_string(),
        }
    }

    pub fn infuse(&mut self, addition: &Ingredient) {
        // TODO
    }

    pub fn add(&mut self, ingredient: Ingredient) {
        for (element, modifiers) in ingredient.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] += ingredient.elements[element][modifier];
            }
        }
    }

    pub fn halve(&mut self) {
        for (element, modifiers) in self.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] = (self.elements[element][modifier] as f32 / 2.0).ceil() as i32;
            }
        }
    }

    pub fn apply(&mut self, ingredient: Ingredient) {
        for (element, modifiers) in ingredient.elements {
            for (modifier, amount) in modifiers {
                let power = self.elements[element][Modifier::Provide];
                match modifier {
                    Modifier::Strengthen => self.elements[element][Modifier::Provide] = (power + amount.min(power)).max(0),
                    Modifier::Stabilize => self.elements[element][Modifier::Stabilize]+= amount,
                    Modifier::Provide => self.elements[element][Modifier::Provide] += amount,
                }
            }
        }
    }

    pub fn advance_time(&mut self) {
        // Water or alcohol evaporate without a container
        if self.container == Container::None {
            match &self.solvent {
                Solvent::Water | Solvent::Ether => self.solvent = Solvent::Air,
                _ => (),
            }
        }
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} base: ", self.solvent)?;
        let any = false;
        for (element, status) in self.elements.iter().filter(|(_, s)| s[Modifier::Provide] != 0) {
            if any {
                write!(f, ", ")?;
            }
            write!(f, "{} {:?}", status[Modifier::Provide], element)?; // TODO: Stability
        }
        Ok(())
    }
}
