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

pub static DANDELION: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Air][Modifier::Provide] = 2;
    elements[Element::Fire][Modifier::Provide] = 1;
    elements[Element::Water][Modifier::Stabilize] = -1;
    Ingredient { solvent: Solvent::Air, container: Container::None, elements }
});

pub static WATER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Water][Modifier::Provide] = 2;
    Ingredient { solvent: Solvent::Water, container: Container::None, elements }
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
    Oil
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
    pub solvent: Solvent,
    pub container: Container,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
}

impl Ingredient {
    pub fn name(&self) -> String {
        // TODO
        "water".to_string()
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
