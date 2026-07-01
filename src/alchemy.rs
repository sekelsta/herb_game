use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{Effect, Element, KnowledgeState, Plant};
use crate::elements::{TAINTABLE_ELEMENTS, EVAPORABLE_ELEMENTS};
use crate::potions::REFERENCE_POTIONS;

pub static WATER: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Water][Modifier::Provide] = 3;
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Water, container: Container::None, elements, toxicity: 0.0, effect: None, strength: 0.0, is_tainted: false, }
});
pub static WINE: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Spirit][Modifier::Provide] = 1;
    elements[Element::Earth][Modifier::Provide] = 1;
    elements[Element::Fire][Modifier::Provide] = 1;
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Wine, container: Container::None, elements, toxicity: 0.0, effect: None, strength: 0.0, is_tainted: false, }
});
pub static SPIRITS: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Spirit][Modifier::Provide] = 3;
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Ethanol, container: Container::None, elements, toxicity: 0.0, effect: None, strength: 0.0, is_tainted: false, }
});
pub static OIL: Lazy<Ingredient> = Lazy::new(|| {
    Ingredient { kind: IngredientKind::BaseSolvent, solvent: Solvent::Oil, container: Container::None, elements: EnumMap::default(), toxicity: 0.0, effect: None, strength: 0.0, is_tainted: false, }
});
pub static ROT: Lazy<Ingredient> = Lazy::new(|| {
    let mut elements: EnumMap<Element, EnumMap<Modifier, i32>> = EnumMap::default();
    elements[Element::Taint][Modifier::Provide] = 3;
    elements[Element::Taint][Modifier::Stabilize] = 3;
    Ingredient { kind: IngredientKind::Rot, solvent: Solvent::Water, container: Container::None, elements, toxicity: 1.0, effect: None, strength: 0.0, is_tainted: false, }
});


#[derive(Clone, Copy, Debug, Enum, EnumIter, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Boost, // Weaken if value is negative
    Stabilize, // Destabilize if value is negative
    Provide,
    //Join,
    //Split,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Solvent {
    Air,
    Water,
    Wine,
    Ethanol,
    Oil,
    Vivo, // Plant matter
}

impl Solvent {
    pub fn name(&self) -> &'static str {
        match self {
            Solvent::Air => "air",
            Solvent::Water => "water",
            Solvent::Wine => "wine",
            Solvent::Ethanol => "spirits",
            Solvent::Oil => "neutral oil",
            Solvent::Vivo => "mashed vegetable", // This shouldn't happen
        }
    }

    pub fn name_as_base(&self) -> &'static str {
        match self {
            Solvent::Air => "Dry mix",
            Solvent::Water => "Water base",
            Solvent::Wine => "Wine base",
            Solvent::Ethanol => "In spirits",
            Solvent::Oil => "In neutral oil",
            Solvent::Vivo => "Mashed vegetable", // This shouldn't happen
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Container {
    Bottle,
    None,
}

impl Container {
    pub fn sale_value(&self) -> i32 {
        match self {
            Container::Bottle => 2,
            Container::None => 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IngredientKind {
    BaseSolvent,
    Herb { species: Plant },
    Decoction { names: Vec<String> },
    Infusion { names: Vec<String> },
    Mixture,
    Rot,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub kind: IngredientKind,
    pub solvent: Solvent,
    pub effect: Option<Effect>,
    pub strength: f32,
    pub container: Container,
    pub elements: EnumMap<Element, EnumMap<Modifier, i32>>,
    pub toxicity: f32,
    pub is_tainted: bool,
}

static ALL_TRUE: Lazy<EnumMap<Element, EnumMap<Modifier, bool>>> = Lazy::new(|| enum_map! {
    _ => enum_map! { _ => true },
});
impl Ingredient {
    pub fn base_name(&self) -> String {
        if let Some(effect) = self.effect {
            return effect.potion_name(&self.solvent);
        }
        match &self.kind {
            IngredientKind::BaseSolvent => self.solvent.name().to_string(),
            IngredientKind::Herb { species } => species.to_lowercase_string(),
            IngredientKind::Infusion { names } | IngredientKind::Decoction { names } => names.join(", "),
            IngredientKind::Mixture => "concoction".to_string(),
            IngredientKind::Rot => "rot".to_string(),
        }
    }

    pub fn brew_name(&self) -> String {
        if let Some(effect) = self.effect {
            return format!("{} ({}% strength)", effect.potion_name(&self.solvent), (self.strength * 100.0).round() as i32);
        }
        match &self.kind {
            IngredientKind::BaseSolvent => self.solvent.name().to_string(),
            IngredientKind::Herb { species } => match self.solvent {
                Solvent::Air => format!("dry {}", species.to_lowercase_string()),
                Solvent::Water => format!("aqueous {}", species.to_lowercase_string()),
                Solvent::Wine => format!("{} wine", species.to_lowercase_string()),
                Solvent::Ethanol => format!("spirit of {}", species.to_lowercase_string()),
                Solvent::Oil => format!("{} oil", species.to_lowercase_string()),
                Solvent::Vivo => format!("fresh {}", species.to_lowercase_string()),
            },
            IngredientKind::Infusion { names } => match self.solvent {
                Solvent::Air => format!("dried infusion of {}", names.join(", ")),
                Solvent::Water => format!("infusion of {}", names.join(", ")),
                Solvent::Wine => format!("wine infusion of {}", names.join(", ")),
                Solvent::Ethanol => format!("{} tincture", names.join(", ")),
                Solvent::Oil => format!("oil of {}", names.join(", ")),
                Solvent::Vivo => format!("mashed {}", names.join(", ")),
            },
            IngredientKind::Decoction { names } => match self.solvent {
                Solvent::Air => format!("dried {} tea", names.join(", ")),
                Solvent::Water => format!("{} tea", names.join(", ")),
                Solvent::Wine => format!("wine decoction of {}", names.join(", ")),
                Solvent::Ethanol => format!("burning water of {}", names.join(", ")),
                Solvent::Oil => format!("boiled {} oil", names.join(", ")),
                Solvent::Vivo => format!("cooked {}", names.join(", ")),
            },
            IngredientKind::Mixture => "herbal concoction".to_string(),
            IngredientKind::Rot => self.base_name(),
        }
    }

    pub fn full_name(&self) -> String {
        let name = match self.container {
            Container::Bottle => format!("bottle of {}", self.brew_name()),
            Container::None => self.brew_name(),
        };
        match self.is_tainted {
            true => format!("{} (tainted)", name),
            false => name,
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

        let discovered = if let IngredientKind::Herb { species } = self.kind {
            discoveries.known_elements.get(&species)
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
            // Note this triggers only for herbs. For in-progress potions, all elements present are considered discovered.
            if !Modifier::iter().any(|modifier| discovered[element][modifier] && (discoveries.stability_known() || modifier != Modifier::Stabilize)) {
                any_unknown = true;
                continue;
            }

            // Theoretically this shouldn't be hit because all mechanisms leaving an existing provide undiscovered should also leave all others undiscovered,
            // showing "Elemental affinities unknown" from above
            if !discovered[element][Modifier::Provide] && status[Modifier::Provide] != 0 {
                any_unknown = true;
                continue;
            }

            let provide = if discovered[element][Modifier::Provide] { status[Modifier::Provide] } else { 0 };
            let strengthen = if discovered[element][Modifier::Boost] { status[Modifier::Boost] } else { 0 };
            let stability = if discovered[element][Modifier::Stabilize] && discoveries.stability_known() { status[Modifier::Stabilize] } else { 0 };
            if provide == 0 && strengthen == 0 && stability == 0 {
                // Skip showing this as unknown inside the cauldron
                continue;
            }

            if any_known {
                string.push_str(", ");
            }
            any_known = true;


            match (provide != 0, strengthen == 0, stability == 0) {
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

    pub fn display_hint(&self, discoveries: &KnowledgeState) -> &'static str {
        if let Some(effect) = self.effect {
            if let Some(potion) = REFERENCE_POTIONS.iter().find(|x| x.effect == effect) {
                return potion.display_hint(self, discoveries);
            }
        }
        ""
    }

    pub fn show_in_progress(&self, discoveries: &KnowledgeState) -> String {
        let start = if let Some(effect) = self.effect {
            format!("{:?} base: {}. Effect: {} ({}% strength)", self.solvent, self.display_elements(discoveries), effect.to_title_case(), (self.strength * 100.0).round() as i32)
        } else {
            return format!("{}: {}. No effect.", self.solvent.name_as_base(), self.display_elements(discoveries))
        };
        format!("{} {}", start, self.display_hint(discoveries))
    }

    pub fn matches_name(&self, needle: &str) -> bool {
        needle.starts_with(self.full_name().to_ascii_lowercase().as_str()) || needle.starts_with(self.brew_name().to_ascii_lowercase().as_str()) || needle.starts_with(self.base_name().to_ascii_lowercase().as_str())
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
        let base_value = match self.effect {
            Some(effect) => (self.strength * self.strength * effect.sale_value() as f32).ceil() as i32,
            None => 0
        };
        // Don't include container value, because the bottles will be returned
        base_value// + self.container.sale_value()
    }

    pub fn is_unstable(&self, element: Element) -> bool {
        let modifiers = &self.elements[element];
        let provide = modifiers[Modifier::Provide];
        return provide > 0 && provide - modifiers[Modifier::Stabilize] > element.base_stability();
    }

    pub fn boil(&mut self, discoveries: &mut KnowledgeState) -> String {
        // Evaporation
        let mut evaporated = None;
        for e in EVAPORABLE_ELEMENTS {
            let provide = self.elements[e][Modifier::Provide];
            if provide > 0 && provide > self.elements[e][Modifier::Stabilize] {
                evaporated = Some(e);
                self.elements[e][Modifier::Provide] -= 1;
                break;
            } else if self.elements[e][Modifier::Boost] > 0 {
                evaporated = Some(e);
                self.elements[e][Modifier::Boost] -= 1;
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

    pub fn taint(&mut self, _discoveries: &mut KnowledgeState) {
        self.elements[Element::Taint][Modifier::Provide] += 1;
        self.is_tainted = true;
        // Don't update effect, because this is usually called on raw ingredients, not brews
        // Could go either way on automatically discovering taint on that ingredient, but for now, leave it out in case it started with taint too
    }

    pub fn decoct(&mut self, addition: &Ingredient, discoveries: &mut KnowledgeState) -> String {
        let boil_text = self.boil(discoveries);
        match (&mut self.kind, &addition.kind) {
            (IngredientKind::BaseSolvent, IngredientKind::Herb { species }) => self.kind = IngredientKind::Decoction { names: vec!(species.to_lowercase_string()) },
            (IngredientKind::Decoction { names }, IngredientKind::Herb { species }) => names.push(species.to_lowercase_string()),
            (IngredientKind::Decoction { names: base_names }, IngredientKind::Decoction { names: addition_names }) => base_names.append(&mut addition_names.clone()),
            _ => self.kind = IngredientKind::Mixture,
        };
        self.apply(addition, discoveries);
        format!("{} You add your {}.\n{}", boil_text, addition.brew_name(), self.show_in_progress(discoveries))
    }

    pub fn infusion_kind(&self, addition: &Ingredient) -> Result<IngredientKind, String> {
        match self.solvent {
            Solvent::Vivo | Solvent::Air => return Err(format!("The base for the infusion must be a liquid, not {}.", self.full_name())),
            Solvent::Water | Solvent::Wine | Solvent::Ethanol | Solvent::Oil => (),
        }
        match addition.solvent {
            Solvent::Air | Solvent::Vivo => (),
            Solvent::Water | Solvent::Wine | Solvent::Ethanol | Solvent::Oil => return Err(format!("You can't soak {} because it's already a liquid.", addition.full_name())),
        }

        Ok(match &self.kind {
            IngredientKind::BaseSolvent => match addition.kind {
                IngredientKind::Herb { species } => IngredientKind::Infusion { names: vec!(species.to_lowercase_string()) },
                IngredientKind::Rot => IngredientKind::Infusion { names: vec!(addition.base_name()) },
                _ => IngredientKind::Mixture,
            },
            IngredientKind::Infusion { names } => match addition.kind {
                IngredientKind::Herb { species } => {
                    let mut names = names.clone();
                    names.push(species.to_lowercase_string());
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
            for (modifier, amount) in modifiers {
                if amount != 0 {
                    match ingredient.kind {
                        IngredientKind::Herb { species } => {
                            let map = discoveries.known_elements.entry(species).or_insert(EnumMap::default());
                            map[element][modifier] = true;
                        },
                        _ => (),
                    }
                }
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
            // TODO: These are not necessarily being checked in a consistent order
            for (modifier, amount) in modifiers {
                let power = self.elements[element][Modifier::Provide];
                let before = self.elements[element][modifier];
                match modifier {
                    Modifier::Boost => self.elements[element][Modifier::Provide] = (power + amount.min(power)).max(0),
                    Modifier::Stabilize => self.elements[element][Modifier::Stabilize] += amount,
                    Modifier::Provide => self.elements[element][Modifier::Provide] += amount,
                }
                if before != self.elements[element][modifier] && (discoveries.stability_known() || modifier != Modifier::Stabilize)
                        || (modifier == Modifier::Boost && power > 0 && ingredient.elements[element][Modifier::Boost] != 0) {
                    match ingredient.kind {
                        IngredientKind::Herb { species } => {
                            let map = discoveries.known_elements.entry(species).or_insert(EnumMap::default());
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
        self.effect = None;
        self.strength = 0.0;
        for potion in &*REFERENCE_POTIONS {
            let effectiveness = potion.calc_strength(self);
            if effectiveness > self.strength {
                self.strength = effectiveness;
                self.effect = Some(potion.effect);
            }
        }
        discoveries.mark_recipe(self);
    }

    // TODO: Fix herb drying spam
    pub fn advance_time(&mut self) -> Option<String> {
        let old_name = self.full_name();
        // Water or alcohol evaporate without a container
        match self.container {
            Container::None => match &self.solvent {
                Solvent::Water | Solvent::Wine | Solvent::Ethanol | Solvent::Vivo => {
                    self.solvent = Solvent::Air;
                    if self.elements[Element::Water][Modifier::Provide] > 0 {
                        self.elements[Element::Water][Modifier::Provide] -= 1;
                        self.elements[Element::Water][Modifier::Stabilize] += 1;
                    }
                    Some(format!("{0} dried into {1}", old_name, self.full_name()))
                }
                Solvent::Air | Solvent::Oil => None,
            },
            Container::Bottle => if let Solvent::Vivo = self.solvent {
                *self = ROT.clone();
                self.container = Container::Bottle;
                Some(format!("{} rotted", old_name))
            } else { None }
        }
    }
}
