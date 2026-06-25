use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{Effect, KnowledgeState};
use crate::potions::REFERENCE_POTIONS;

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
    elements[Element::Water][Modifier::Provide] = 3;
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Water, container: Container::None, elements, toxicity: 0.0, }
});
pub static ETHER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Spirit][Modifier::Provide] = 3;
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Ether, container: Container::None, elements, toxicity: 0.0, }
});
pub static OIL: Lazy<Ingredient> = Lazy::new(|| {
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Oil, container: Container::None, elements: EnumMap::default(), toxicity: 0.0, }
});
pub static ROT: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Taint][Modifier::Provide] = 3;
    elements[Element::Taint][Modifier::Stabilize] = 3;
    Ingredient { kind: IngredientKind::Rot, solvent: Solvent::Water, container: Container::None, elements, toxicity: 1.0, }
});

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

#[derive(Clone, Copy, Debug, Enum, EnumIter, PartialEq)]
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
    Vivo, // Plant matter
}

impl Solvent {
    pub fn name(&self) -> &'static str {
        match self {
            Solvent::Air => "air",
            Solvent::Water => "water",
            Solvent::Ether => "spirits",
            Solvent::Oil => "neutral oil",
            Solvent::Vivo => "mashed vegetable", // This shouldn't happen
        }
    }
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

#[derive(Clone, Debug)]
pub enum IngredientKind {
    BaseSolvent,
    Potion { effect: Effect, strength: f32 },
    Herb { name: &'static str },
    Decoction { names: Vec<String> },
    Infusion { names: Vec<String> },
    Mixture,
    Rot,
}

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub kind: IngredientKind,
    pub solvent: Solvent,
    pub container: Container,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
    pub toxicity: f32,
}

static ALL_TRUE: Lazy<EnumMap<Element, EnumMap<Modifier, bool>>> = Lazy::new(|| enum_map! {
    _ => enum_map! { _ => true },
});
impl Ingredient {
    pub fn base_name(&self) -> String {
        match &self.kind {
            IngredientKind::BaseSolvent => self.solvent.name().to_string(),
            IngredientKind::Potion { effect, .. } => effect.potion_name(&self.solvent),
            IngredientKind::Herb { name} => name.to_string(),
            IngredientKind::Infusion { names } | IngredientKind::Decoction { names } => names.join(", "),
            IngredientKind::Mixture => "concoction".to_string(),
            IngredientKind::Rot => "rot".to_string(),
        }
    }

    pub fn brew_name(&self) -> String {
        match &self.kind {
            IngredientKind::BaseSolvent => self.solvent.name().to_string(),
            IngredientKind::Potion { effect, strength} => {
                format!("{} ({}% strength)", effect.potion_name(&self.solvent), (strength * 100.0).round() as i32)
            }
            IngredientKind::Herb { name} => match self.solvent {
                Solvent::Air => format!("dry {}", name),
                Solvent::Water => format!("aqueous {}", name),
                Solvent::Ether => format!("spirit of {}", name),
                Solvent::Oil => format!("{} oil", name),
                Solvent::Vivo => format!("fresh {}", name),
            },
            IngredientKind::Infusion { names } => match self.solvent {
                Solvent::Air => format!("dried infusion of {}", names.join(", ")),
                Solvent::Water => format!("infusion of {}", names.join(", ")),
                Solvent::Ether => format!("{} tincture", names.join(", ")),
                Solvent::Oil => format!("oil of {}", names.join(", ")),
                Solvent::Vivo => format!("mashed {}", names.join(", ")),
            },
            IngredientKind::Decoction { names } => match self.solvent {
                Solvent::Air => format!("dried {} tea", names.join(", ")),
                Solvent::Water => format!("{} tea", names.join(", ")),
                Solvent::Ether => format!("boiled {} ether", names.join(", ")),
                Solvent::Oil => format!("boiled {} oil", names.join(", ")),
                Solvent::Vivo => format!("cooked {}", names.join(", ")),
            },
            IngredientKind::Mixture => "herbal concoction".to_string(),
            IngredientKind::Rot => self.base_name(),
        }
    }

    pub fn full_name(&self) -> String {
        match self.container {
            Container::Bottle => format!("bottle of {}", self.brew_name()),
            Container::None => self.brew_name(),
        }
    }

    pub fn inventory_view(&self, discoveries: &KnowledgeState) -> String {
        let value = self.sale_value();
        let elements = self.display_elements(discoveries);
        match value != 0 {
            true => format!("{} - {}. Sell: {}", self.full_name(), elements, value),
            false => format!("{} - {}", self.full_name(), elements),
        }
    }

    fn display_elements(&self, discoveries: &KnowledgeState) -> String {
        let mut string = "".to_string();

        let discovered = if let IngredientKind::Herb { name } = self.kind {
            discoveries.known_elements.get(name)
        } else {
            Some(&*ALL_TRUE)
        };
        let discovered = match discovered {
            Some(map) => map,
            None => return "Elemental affinities unknown".to_string(),
        };

        let mut any_known = false;
        let mut any_unknown = false;
        for (element, status) in self.elements {
            if Modifier::iter().all(|modifier| status[modifier] == 0) {
                continue;
            }
            if !Modifier::iter().any(|modifier| discovered[element][modifier]) {
                any_unknown = true;
                continue;
            }

            if any_known {
                string.push_str(", ");
            }
            any_known = true;

            let provide = if discovered[element][Modifier::Provide] { status[Modifier::Provide] } else { 0 };
            let strengthen = if discovered[element][Modifier::Strengthen] { status[Modifier::Strengthen] } else { 0 };
            let stability = if discovered[element][Modifier::Stabilize] { status[Modifier::Stabilize] } else { 0 };

            match (provide != 0, strengthen == 0, stability == 0 || !discoveries.stability_known) {
                (_, true, true) => string.push_str(format!("{} {:?}", provide, element).as_str()),
                (true, true, false) => string.push_str(format!("{} {:?} ({:+} stability)", provide, element, stability).as_str()),
                (false, true, false) => string.push_str(format!("{:+} {:?} stability", stability, element).as_str()),
                (_, false, true) => string.push_str(format!("{} ({:+}) {:?}", provide, strengthen, element).as_str()),
                (_, false, false) => string.push_str(format!("{} ({:+}) {:?} ({:+} stability)", provide, strengthen, element, stability).as_str()),
            }
        }
        if any_unknown {
            format!("{}, Unknown", string)
        } else if any_known {
            string
        } else {
            "Inert".to_string()
        }
    }

    pub fn show_in_progress(&self, discoveries: &KnowledgeState) -> String {
        match &self.kind {
            IngredientKind::Potion { effect, strength } => format!("{:?} base: {}. Effect: {} ({}% strength)", self.solvent, self.display_elements(discoveries), effect.to_title_case(), (strength * 100.0).round() as i32),
            _ => format!("{:?} base: {}", self.solvent, self.display_elements(discoveries)),
        }
    }

    pub fn matches_name(&self, needle: &str) -> bool {
        needle.starts_with(self.full_name().as_str()) || needle.starts_with(self.brew_name().as_str()) || needle.starts_with(self.base_name().as_str())
    }

    pub fn search_remainder<'a>(&self, needle: &'a str) -> Option<&'a str> {
        let full_name = self.full_name();
        let name = self.brew_name();
        let base_name = self.base_name();
        if needle == full_name || needle == name || needle == base_name {
            return None;
        }
        if needle.starts_with(full_name.as_str()) {
            // Plus one to get rid of the separating space
            return Some(&needle[full_name.len()+1..]);
        }
        if needle.starts_with(name.as_str()) {
            return Some(&needle[name.len()+1..]);
        }
        if needle.starts_with(base_name.as_str()) {
            return Some(&needle[base_name.len()+1..]);
        }
        return Some(needle);
    }

    pub fn sale_value(&self) -> i32 {
        let base_value = match self.kind {
            IngredientKind::Potion { effect, strength } => (strength * strength * effect.sale_value() as f32).round() as i32,
            _ => 0
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
        let boil_text = self.boil(discoveries);
        match (&mut self.kind, &addition.kind) {
            (IngredientKind::BaseSolvent, IngredientKind::Herb { name }) => self.kind = IngredientKind::Decoction { names: vec!(name.to_string()) },
            (IngredientKind::Decoction { names }, IngredientKind::Herb { name }) => names.push(name.to_string()),
            (IngredientKind::Decoction { names: base_names }, IngredientKind::Decoction { names: addition_names }) => base_names.append(&mut addition_names.clone()),
            _ => self.kind = IngredientKind::Mixture,
        };
        self.apply(addition, discoveries);
        format!("{}\n{}", boil_text, self.show_in_progress(discoveries))
    }

    pub fn infusion_kind(&self, addition: &Ingredient) -> Result<IngredientKind, String> {
        match self.solvent {
            Solvent::Vivo | Solvent::Air => return Err(format!("The base for the infusion must be a liquid, not {}.", self.full_name())),
            Solvent::Water | Solvent::Ether | Solvent::Oil => (),
        }
        match addition.solvent {
            Solvent::Air | Solvent::Vivo => (),
            Solvent::Water | Solvent::Ether | Solvent::Oil => return Err(format!("You can't soak {} because it's already a liquid.", addition.full_name())),
        }

        Ok(match &self.kind {
            IngredientKind::BaseSolvent => match addition.kind {
                IngredientKind::Herb { name } => IngredientKind::Infusion { names: vec!(name.to_string()) },
                IngredientKind::Rot => IngredientKind::Infusion { names: vec!(addition.base_name()) },
                _ => IngredientKind::Mixture,
            },
            IngredientKind::Infusion { names } => match addition.kind {
                IngredientKind::Herb { name } => {
                    let mut names = names.clone();
                    names.push(name.to_string());
                    IngredientKind::Infusion { names }
                },
                IngredientKind::Rot => {
                    let mut names = names.clone();
                    names.push(addition.base_name());
                    IngredientKind::Infusion { names }
                },
                _ => IngredientKind::Mixture,
            },
            _ => IngredientKind::Mixture,
        })
    }

    pub fn infuse(&mut self, addition: &Ingredient, discoveries: &mut KnowledgeState) -> String {
        let mut ingredient = addition.clone();
        ingredient.discard_insoluble(&self.solvent);
        ingredient.halve();
        self.add(&ingredient, discoveries);
        self.show_in_progress(discoveries)
    }

    pub fn add(&mut self, ingredient: &Ingredient, discoveries: &mut KnowledgeState) {
        self.toxicity += ingredient.toxicity;
        for (element, modifiers) in ingredient.elements {
            // TODO: Discover strengthening, or only provide?
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
                let before = self.elements[element][modifier];
                match modifier {
                    Modifier::Strengthen => self.elements[element][Modifier::Provide] = (power + amount.min(power)).max(0),
                    Modifier::Stabilize => self.elements[element][Modifier::Stabilize]+= amount,
                    Modifier::Provide => self.elements[element][Modifier::Provide] += amount,
                }
                if before != self.elements[element][modifier] && (discoveries.stability_known || modifier != Modifier::Stabilize) {
                    match ingredient.kind {
                        IngredientKind::Herb { name } => {
                            let map = discoveries.known_elements.entry(name).or_insert(EnumMap::default());
                            map[element][modifier] = true;
                        },
                        _ => (),
                    }
                }
            }
        }
        self.update_effect(discoveries);
    }

    pub fn update_effect(&mut self, discoveries: &mut KnowledgeState) {
        if let IngredientKind::Potion { .. } = self.kind {
            self.kind = IngredientKind::Mixture;
        }
        let mut strength = 0.0;
        for potion in &*REFERENCE_POTIONS {
            let effectiveness = potion.calc_strength(self);
            if effectiveness > strength {
                strength = effectiveness;
                self.kind = IngredientKind::Potion { effect: potion.effect, strength };
            }
        }
        if let IngredientKind::Potion { effect, .. } = self.kind {
            discoveries.effects[effect] = true;
        }
    }

    pub fn advance_time(&mut self) -> Option<String> {
        let old_name = self.full_name();
        // Water or alcohol evaporate without a container
        match self.container {
            Container::None => match &self.solvent {
                Solvent::Water | Solvent::Ether => {
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
