use std::fmt;
use once_cell::sync::Lazy;

use enum_map::{Enum, EnumMap};

const TAINTABLE_ELEMENTS: [Element; 11] = [
    Element::Mana,
    Element::Spirit,
    Element::Shadow,
    Element::Void,
    Element::Thunder,
    Element::Air,
    Element::Ice,
    Element::Light,
    Element::Fire,
    Element::Water,
    Element::Earth,
];

const EVAPORABLE_ELEMENTS: [Element; 7] = [
    Element::Void,
    Element::Air,
    Element::Spirit,
    Element::Light,
    Element::Fire,
    Element::Water,
    Element::Mana,
];

// Names are hardcoded in name() so make sure to adjust both places if changing
pub static WATER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Water][Modifier::Provide] = 4;
    Ingredient { name: "water", solvent: Solvent::Water, container: Container::None, elements, toxicity: 0.0, effect: None, strength: 0.0, }
});
pub static ETHER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Spirit][Modifier::Provide] = 4;
    Ingredient { name: "spirits", solvent: Solvent::Ether, container: Container::None, elements, toxicity: 0.0, effect: None, strength: 0.0, }
});
pub static OIL: Lazy<Ingredient> = Lazy::new(|| {
    Ingredient { name: "neutral oil", solvent: Solvent::Oil, container: Container::None, elements: EnumMap::default(), toxicity: 0.0, effect: None, strength: 0.0, }
});
pub static ROT: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Taint][Modifier::Provide] = 4;
    elements[Element::Taint][Modifier::Stabilize] = 4;
    Ingredient { name: "rot", solvent: Solvent::Water, container: Container::None, elements, toxicity: 0.0, effect: None, strength: 0.0, }
});

pub static REFERENCE_POTIONS: [Lazy<Ingredient>; 1] = [
    Lazy::new(|| {
        let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
        elements[Element::Taint][Modifier::Provide] = 4;
        Ingredient { name: "potion of testing", solvent: Solvent::Water, container: Container::Bottle, elements, toxicity: 0.0, effect: Some(Effect::CoughRemedy), strength: 1.0, }
    })
];

#[derive(Clone, Copy, Debug, strum_macros::Display, Enum, PartialEq)]
pub enum Element {
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

impl Element {
    fn soluble(&self, solvent: &Solvent) -> bool {
        use Element::*;
        match solvent {
            Solvent::Water => !matches!(self, Earth | Taint | Mana),
            Solvent::Ether => !matches!(self, Earth | Thunder),
            Solvent::Oil => matches!(self, Void | Air | Taint | Light | Shadow),
            Solvent::Air => matches!(self, Void | Air | Spirit | Light),
            Solvent::Vivo => true,
        }
    }
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

#[derive(Clone, Copy, Debug)]
pub enum Effect {
    CoughRemedy,
}

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub name: &'static str,
    pub solvent: Solvent,
    pub container: Container,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
    pub effect: Option<Effect>,
    pub strength: f32,
    pub toxicity: f32,
}

impl Ingredient {
    pub fn new_herb(name: &'static str, toxicity: f32, f: impl Fn(&mut EnumMap<Element, EnumMap<Modifier, i32>>)) -> Self {
        let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
        f(&mut elements);
        Self {
            name,
            solvent: Solvent::Vivo,
            container: Container::None,
            elements,
            effect: None,
            strength: 0.0,
            toxicity,
        }
    }

    pub fn full_name(&self) -> String {
        let name = match self.solvent {
            Solvent::Air => format!("dry {}", self.name),
            Solvent::Ether if self.name != "spirits" => format!("{} tincture", self.name),
            Solvent::Water if self.name != "water" && self.name != "rot" => format!("aqueous {}", self.name),
            Solvent::Oil if self.name != "neutral oil" => format!("{} oil", self.name),
            Solvent::Vivo => format!("fresh {}", self.name),
            _ => self.name.to_string(),
        };
        match self.container {
            Container::Bottle => format!("bottle of {}", name),
            Container::None => name,
        }
    }

    pub fn inventory_view(&self) -> String {
        format!("{} - {}", self.full_name(), self.display_elements())
    }

    fn display_elements(&self) -> String {
        let mut string = "".to_string();
        let mut any = false;
        for (element, status) in self.elements.iter().filter(|(_, s)| s[Modifier::Provide] != 0) {
            if any {
                string.push_str(", ");
            }
            any = true;
            if status[Modifier::Stabilize] == 0 {
                string.push_str(format!("{} {:?}", status[Modifier::Provide], element).as_str());
            } else if status[Modifier::Stabilize] > 0 {
                string.push_str(format!("{} {:?} (+{} stability)", status[Modifier::Provide], element, status[Modifier::Stabilize]).as_str());
            } else {
                string.push_str(format!("{} {:?} ({} stability)", status[Modifier::Provide], element, status[Modifier::Stabilize]).as_str());
            }
        }
        if any {
            string
        } else {
            "Inert".to_string()
        }
    }

    pub fn show_in_progress(&self) -> String {
        format!("{:?} base: {}. Effect: {:?} ({}% strength)", self.solvent, self.display_elements(), self.effect, self.strength * 100.0)
    }

    pub fn matches_name(&self, needle: &str) -> bool {
        needle.starts_with(self.full_name().as_str()) || needle.starts_with(self.name)
    }

    pub fn search_remainder<'a>(&self, needle: &'a str) -> Option<&'a str> {
        let full_name = self.full_name();
        if needle == full_name || needle == self.name {
            return None;
        }
        if needle.starts_with(full_name.as_str()) {
            // Plus one to get rid of the separating space
            return Some(&needle[full_name.len()+1..]);
        }
        if needle.starts_with(self.name) {
            return Some(&needle[self.name.len()+1..]);
        }
        return Some(needle);
    }

    pub fn boil(&mut self) -> String {
        // Evaporation
        let mut evaporated = None;
        for e in EVAPORABLE_ELEMENTS {
            if self.elements[e][Modifier::Provide] > 0 {
                evaporated = Some(e);
                self.elements[e][Modifier::Provide] -= 1;
                break;
            } else if self.elements[e][Modifier::Strengthen] > 0 {
                evaporated = Some(e);
                self.elements[e][Modifier::Strengthen] -= 1;
                break;
            }
        }
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
        self.update_effect();
        match (taint_spread, evaporated) {
            (false, None) => "The cauldron boils.".to_string(),
            (false, Some(e)) => format!("The cauldron boils. Elemental {} evaporates.", e.to_string().to_lowercase()),
            (true, None) => "The cauldron boils. Taint spreads.".to_string(),
            (true, Some(e)) => format!("The cauldron boils. Elemental {} evaporates. Taint spreads.", e.to_string().to_lowercase()),
        }
    }

    pub fn decoct(&mut self, addition: &Ingredient) -> String {
        format!("{}\n{}", self.boil(), { self.apply(addition); self.show_in_progress() })
    }

    pub fn infuse(&mut self, addition: &Ingredient) -> String {
        self.name = addition.name;
        let mut ingredient = addition.clone();
        ingredient.discard_insoluble(&self.solvent);
        ingredient.halve();
        self.add(&ingredient);
        self.show_in_progress()
    }

    pub fn add(&mut self, ingredient: &Ingredient) {
        self.toxicity += ingredient.toxicity;
        for (element, modifiers) in ingredient.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] += amount;
            }
        }
        self.update_effect();
    }

    pub fn halve(&mut self) {
        for (element, modifiers) in self.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] = (amount as f32 / 2.0).ceil() as i32;
            }
        }
    }

    fn discard_insoluble(&mut self, solvent: &Solvent) {
        for (element, modifiers) in self.elements {
            if element == Element::Taint {
                // You're not getting rid of it that easy
                for (modifier, amount) in modifiers {
                    self.elements[element][modifier] = (amount as f32 / 2.0).ceil() as i32;
                }
            }
            else if !element.soluble(solvent) {
                for (modifier, _) in modifiers {
                    self.elements[element][modifier] = 0;
                }
            }
        }
    }

    pub fn apply(&mut self, ingredient: &Ingredient) {
        self.toxicity += ingredient.toxicity;
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
        self.update_effect();
    }

    pub fn update_effect(&mut self) {
        for potion in &REFERENCE_POTIONS {
            let effectiveness = self.calc_strength(&potion);
            if effectiveness > self.strength {
                self.strength = effectiveness;
                self.effect = potion.effect;
                self.name = potion.name;
            }
        }
    }

    pub fn calc_strength(&self, reference: &Ingredient) -> f32 {
        let mut ref_total = 0;
        let mut ratio: f32 = 10.0; // Max strength before being more concentrated starts counting against you even in the correct ratio
        for (element, modifiers) in reference.elements {
            let theirs = modifiers[Modifier::Provide];
            let ours = self.elements[element][Modifier::Provide];
            if theirs > ours {
                return 0.0;
            }
            ref_total += theirs;
            ratio = ratio.min(ours as f32 / theirs as f32);
        }
        let mut our_total = 0;
        for (_element, modifiers) in self.elements {
            our_total += modifiers[Modifier::Provide];
        }
        return ref_total as f32 * ratio / our_total as f32;
    }

    pub fn advance_time(&mut self) -> Option<String> {
        let old_name = self.full_name();
        // Water or alcohol evaporate without a container
        match self.container {
            Container::None => match &self.solvent {
                Solvent::Water | Solvent::Ether | Solvent::Vivo => {
                    self.solvent = Solvent::Air;
                    Some(format!("{0} dried into {1}", old_name, self.full_name()))
                }
                _ => None,
            },
            Container::Bottle => if let Solvent::Vivo = self.solvent {
                *self = ROT.clone();
                self.container = Container::Bottle;
                Some(format!("{} rotted", old_name))
            } else { None }
        }
    }
}
