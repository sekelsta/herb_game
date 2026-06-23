use std::fmt;
use enum_map::{Enum, EnumMap};
use once_cell::sync::Lazy;

use crate::KnowledgeState;

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

const EVAPORABLE_ELEMENTS: [Element; 8] = [
    Element::Void,
    Element::Air,
    Element::Spirit,
    Element::Light,
    Element::Fire,
    Element::Shadow,
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

use Element::*;
use Effect::*;
pub static REFERENCE_POTIONS: [Lazy<Ingredient>; 29] = [
    Lazy::new(|| Ingredient::new_potion("cough remedy", CoughRemedy, 1.0, |elements| {
        elements[Ice] = 2;
        elements[Thunder] = 2;
        elements[Air] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("fever reducer", FeverReducer, 1.0, |elements| {
        elements[Ice] = 3;
        elements[Water] = 2;
        elements[Shadow] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("salve of healing", WoundHealing, 1.0, |elements| {
        elements[Earth] = 4;
        elements[Air] = 4;
    })),
    Lazy::new(|| Ingredient::new_potion("insect repellent", InsectRepellent, 1.0, |elements| {
        elements[Light] = 4;
        elements[Air] = 3;
        elements[Fire] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("snake repellent", SnakeRepellent, 1.0, |elements| {
        elements[Ice] = 3;
        elements[Fire] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("protection from charms", CharmProtection, 1.0, |elements| {
        elements[Fire] = 4;
        elements[Earth] = 4;
    })),
    Lazy::new(|| Ingredient::new_potion("love potion", Love, 1.0, |elements| {
        elements[Fire] = 3;
        elements[Air] = 3;
        elements[Ice] = 1;
        elements[Thunder] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of cleanliness", Cleanliness, 1.0, |elements| {
        elements[Void] = 6;
        elements[Air] = 3;
        elements[Light] = 2;
        elements[Ice] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of fear", Fear, 1.0, |elements| {
        elements[Ice] = 3;
        elements[Shadow] = 3;
        elements[Water] = 2;
        elements[Thunder] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of rage", Rage, 1.0, |elements| {
        elements[Fire] = 3;
        elements[Shadow] = 3;
        elements[Taint] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of courage", Courage, 1.0, |elements| {
        elements[Spirit] = 3;
        elements[Fire] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("relaxant", Relaxation, 1.0, |elements| {
        elements[Water] = 3;
        elements[Fire] = 2;
        elements[Earth] = 1;
        elements[Light] = 1;
        elements[Spirit] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("sleep draught", Sleep, 1.0, |elements| {
        elements[Water] = 3;
        elements[Taint] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("paralyzing poison", Paralysis, 1.0, |elements| {
        elements[Taint] = 3;
        elements[Void] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("intelligence potion", Intelligence, 1.0, |elements| {
        elements[Water] = 5;
        elements[Spirit] = 4;
        elements[Fire] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("strength potion", Strength, 1.0, |elements| {
        elements[Earth] = 6;
        elements[Thunder] = 2;
        elements[Ice] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of patience", Patience, 1.0, |elements| {
        elements[Earth] = 5;
        elements[Water] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of plant growth", PlantGrowth, 1.0, |elements| {
        elements[Water] = 4;
        elements[Light] = 4;
        elements[Earth] = 2;
        elements[Air] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of resillience", Resillience, 1.0, |elements| {
        elements[Earth] = 4;
        elements[Shadow] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("speed booster", Speed, 1.0, |elements| {
        elements[Air] = 6;
        elements[Thunder] = 6;
        elements[Light] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("charisma", Charisma, 1.0, |elements| {
        elements[Spirit] = 6;
        elements[Void] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("potion of seeing", Perception, 1.0, |elements| {
        elements[Light] = 5;
        elements[Shadow] = 4;
    })),
    Lazy::new(|| Ingredient::new_potion("perfume of loveliness", Loveliness, 1.0, |elements| {
        elements[Air] = 7;
        elements[Fire] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("vial of shock", Shock, 1.0, |elements| {
        elements[Light] = 5;
        elements[Fire] = 4;
        elements[Thunder] = 3;
    })),
    Lazy::new(|| Ingredient::new_potion("vial of fire", Flame, 1.0, |elements| {
        elements[Fire] = 8;
        elements[Earth] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("vial of lightning", Lightning, 1.0, |elements| {
        elements[Thunder] = 6;
        elements[Light] = 6;
    })),
    Lazy::new(|| Ingredient::new_potion("vial of ice", Freeze, 1.0, |elements| {
        elements[Ice] = 8;
        elements[Earth] = 1;
    })),
    Lazy::new(|| Ingredient::new_potion("vial of poison", Poison, 1.0, |elements| {
        elements[Taint] = 6;
        elements[Air] = 2;
    })),
    Lazy::new(|| Ingredient::new_potion("vial of darkness", Darkness, 1.0, |elements| {
        elements[Shadow] = 6;
        elements[Taint] = 1;
        elements[Thunder] = 1;
    })),
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

impl Container {
    pub fn sale_value(&self) -> i32 {
        match self {
            Container::Bottle => 1,
            Container::None => 0,
        }
    }
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

#[derive(Clone, Copy, Debug, strum_macros::Display, Enum, PartialEq)]
pub enum Effect {
    CoughRemedy,
    FeverReducer,
    InsectRepellent,
    SnakeRepellent,
    CharmProtection,
    PlantGrowth,
    WoundHealing,
    Love,
    Fear,
    Rage,
    Courage,
    Relaxation,
    Sleep,
    Paralysis,
    Intelligence,
    Strength,
    Patience,
    Resillience,
    Speed,
    Charisma,
    Perception,
    Loveliness,
    Cleanliness,
    Flame, 
    Lightning,
    Freeze,
    Shock,
    Poison,
    Darkness,
}

impl Effect {
    fn to_title_case(&self) -> String {
        use Effect::*;
        match self {
            CoughRemedy => "Cough Remedy".to_string(),
            FeverReducer => "Fever Reducer".to_string(),
            InsectRepellent => "Insect Repellent".to_string(),
            SnakeRepellent => "Snake Repellent".to_string(),
            CharmProtection => "Charm Protection".to_string(),
            PlantGrowth => "Plant Growth".to_string(),
            WoundHealing => "Wound Healing".to_string(),
            e => e.to_string(),
        }
    }

    pub fn sale_value(&self) -> i32 {
        use Effect::*;
        match self {
            CoughRemedy => 3,
            FeverReducer => 4,
            InsectRepellent => 5,
            SnakeRepellent => 6,
            CharmProtection => 7,
            PlantGrowth => 4,
            WoundHealing => 8,
            Love => 8,
            Fear => 9,
            Rage => 12,
            Courage => 12,
            Relaxation => 12,
            Sleep => 12,
            Paralysis => 14,
            Intelligence => 16,
            Strength => 14,
            Patience => 12,
            Resillience => 15,
            Speed => 14,
            Charisma => 18,
            Perception => 15,
            Loveliness => 22,
            Cleanliness => 18,
            Flame => 24, 
            Lightning => 25,
            Freeze => 26,
            Shock => 23,
            Poison => 13,
            Darkness => 28,
        }
    }
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
    pub fn new_potion(name: &'static str, effect: Effect, strength: f32, f: impl Fn(&mut EnumMap<Element, i32>)) -> Self {
        let mut elements_provided: EnumMap<Element, i32> = EnumMap::default();
        f(&mut elements_provided);
        let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
        for (element, amount) in elements_provided {
            elements[element][Modifier::Provide] = amount;
        }
        Self {
            name,
            solvent: Solvent::Water,
            container: Container::Bottle,
            elements,
            effect: Some(effect),
            strength,
            toxicity: 0.0,
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
        let value = self.sale_value();
        if value != 0 {
            format!("{} - {}. Sell: {}", self.full_name(), self.display_elements(), value)
        } else {
            format!("{} - {}", self.full_name(), self.display_elements())
        }
    }

    fn display_elements(&self) -> String {
        let mut string = "".to_string();
        let mut any = false;
        for (element, status) in self.elements {
            let provide = status[Modifier::Provide];
            let stability = status[Modifier::Stabilize];
            let strengthen = status[Modifier::Strengthen];
            if provide == 0 && stability == 0 && strengthen == 0 {
                continue;
            }
            if any {
                string.push_str(", ");
            }
            any = true;
            match (strengthen == 0, stability == 0) {
                (true, true) => string.push_str(format!("{} {:?}", provide, element).as_str()),
                (true, false) => string.push_str(format!("{} {:?} ({:+} stability)", provide, element, stability).as_str()),
                (false, true) => string.push_str(format!("{} ({:+}) {:?}", provide, strengthen, element).as_str()),
                (false, false) => string.push_str(format!("{} ({:+}) {:?} ({:+} stability)", provide, strengthen, element, stability).as_str()),
            }
        }
        if any {
            string
        } else {
            "Inert".to_string()
        }
    }

    pub fn show_in_progress(&self) -> String {
        match &self.effect {
            Some(effect) => format!("{:?} base: {}. Effect: {} ({:.1}% strength)", self.solvent, self.display_elements(), effect.to_title_case(), self.strength * 100.0),
            None => format!("{:?} base: {}", self.solvent, self.display_elements()),
        }
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

    pub fn sale_value(&self) -> i32 {
        let base_value = match self.effect {
            Some(effect) => (self.strength * self.strength * effect.sale_value() as f32).round() as i32,
            None => 0
        };
        base_value + self.container.sale_value()
    }

    pub fn boil(&mut self, discoveries: &mut KnowledgeState) -> String {
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
        self.update_effect(discoveries);
        match (taint_spread, evaporated) {
            (false, None) => "The cauldron boils.".to_string(),
            (false, Some(e)) => format!("The cauldron boils. Elemental {} evaporates.", e.to_string().to_lowercase()),
            (true, None) => "The cauldron boils. Taint spreads.".to_string(),
            (true, Some(e)) => format!("The cauldron boils. Elemental {} evaporates. Taint spreads.", e.to_string().to_lowercase()),
        }
    }

    pub fn decoct(&mut self, addition: &Ingredient, discoveries: &mut KnowledgeState) -> String {
        format!("{}\n{}", self.boil(discoveries), { self.apply(addition, discoveries); self.show_in_progress() })
    }

    pub fn infuse(&mut self, addition: &Ingredient, discoveries: &mut KnowledgeState) -> String {
        self.name = addition.name;
        let mut ingredient = addition.clone();
        ingredient.discard_insoluble(&self.solvent);
        ingredient.halve();
        self.add(&ingredient, discoveries);
        self.show_in_progress()
    }

    pub fn add(&mut self, ingredient: &Ingredient, discoveries: &mut KnowledgeState) {
        self.toxicity += ingredient.toxicity;
        for (element, modifiers) in ingredient.elements {
            for (modifier, amount) in modifiers {
                self.elements[element][modifier] += amount;
            }
        }
        self.update_effect(discoveries);
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

    pub fn apply(&mut self, ingredient: &Ingredient, discoveries: &mut KnowledgeState) {
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
        self.update_effect(discoveries);
    }

    pub fn update_effect(&mut self, discoveries: &mut KnowledgeState) {
        for potion in &REFERENCE_POTIONS {
            let effectiveness = self.calc_strength(&potion);
            if effectiveness > self.strength {
                self.strength = effectiveness;
                self.effect = potion.effect;
                self.name = potion.name;
                discoveries.effects[potion.effect.expect("All reference potions have an effect")] = true;
            }
        }
    }

    pub fn calc_strength(&self, reference: &Ingredient) -> f32 {
        let mut ref_total = 0;
        let mut correct_total = 0;
        let mut incorrect_total = 0;
        let mut ratio: f32 = 10.0; // Max strength before being more concentrated starts counting against you even in the correct ratio
        for (element, modifiers) in reference.elements {
            let theirs = modifiers[Modifier::Provide];
            let ours = self.elements[element][Modifier::Provide];
            // Allow up to one of each element to be missing
            if theirs > ours + 1 {
                return 0.0;
            }
            ref_total += theirs;
            correct_total += theirs.min(ours);
            incorrect_total += (theirs - ours).abs();
            ratio = ratio.min(ours as f32 / theirs as f32).max(1.0);
        }
        let mut our_total = 0;
        for (_element, modifiers) in self.elements {
            our_total += modifiers[Modifier::Provide];
        }
        let strength = if correct_total < ref_total {
            (correct_total - incorrect_total).max(0) as f32 / ref_total as f32
        } else {
            ref_total as f32 * ratio / our_total as f32
        };
        return strength * reference.strength;
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
