use once_cell::sync::Lazy;
use enum_map::{Enum, EnumMap};

use crate::{Element, Ingredient, Modifier};
use Element::*;
use Effect::*;

#[derive(Clone, Copy, Debug, strum_macros::Display, Enum, PartialEq)]
pub enum Effect {
    HealthBoost,
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
    pub fn to_title_case(&self) -> String {
        match self {
            HealthBoost => "Health Boost".to_string(),
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
        match self {
            HealthBoost => 3,
            CoughRemedy => 5,
            FeverReducer => 6,
            InsectRepellent => 7,
            SnakeRepellent => 8,
            CharmProtection => 9,
            PlantGrowth => 6,
            WoundHealing => 10,
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
pub struct Potion {
    pub name: &'static str,
    pub effect: Effect,
    pub elements: EnumMap<Element, i32>,
    //pub min_toxicity: f32,
    //pub max_toxicity: f32,
}

impl Potion {
    pub fn new(name: &'static str, effect: Effect, f: impl Fn(&mut EnumMap<Element, i32>)) -> Self {
        let mut elements: EnumMap<Element, i32> = EnumMap::default();
        f(&mut elements);
        Self {
            name,
            effect,
            elements,
            //min_toxicity: 0.0,
            //max_toxicity: 1.0,
        }
    }

    pub fn calc_strength(&self, work: &Ingredient) -> f32 {
        let mut ref_total = 0;
        let mut correct_total = 0;
        let mut incorrect_total = 0;
        let mut ratio: f32 = 10.0; // Max strength before being more concentrated starts counting against you even in the correct ratio
        for (element, required) in self.elements {
            let provided = work.elements[element][Modifier::Provide];
            // Allow up to one of each element to be missing
            if required > provided + 1 {
                return 0.0;
            }
            ref_total += required;
            correct_total += required.min(provided);
            incorrect_total += (required - provided).abs();
            ratio = ratio.min(provided as f32 / required as f32).max(1.0);
        }
        let mut provided_total = 0;
        for (_element, modifiers) in work.elements {
            provided_total += modifiers[Modifier::Provide];
        }

        if correct_total < ref_total {
            (correct_total - incorrect_total).max(0) as f32 / ref_total as f32
        } else {
            ref_total as f32 * ratio / provided_total as f32
        }
    }
}

pub static REFERENCE_POTIONS: Lazy<Vec<Potion>> = Lazy::new(|| vec!(
    // ------ Craftable with only basic elements ------ //
    Potion::new("health tonic", HealthBoost, |elements| {
        elements[Earth] = 4;
        elements[Air] = 4;
    }),
    Potion::new("love potion", Love, |elements| {
        elements[Fire] = 3;
        elements[Air] = 3;
        elements[Ice] = 1;
        elements[Thunder] = 1;
    }),
    Potion::new("relaxant", Relaxation, |elements| {
        elements[Water] = 3;
        elements[Fire] = 2;
        elements[Earth] = 1;
        elements[Light] = 1;
        elements[Spirit] = 1;
    }),
    Potion::new("potion of patience", Patience, |elements| {
        elements[Earth] = 5;
        elements[Water] = 3;
    }),
    Potion::new("perfume of loveliness", Loveliness, |elements| {
        elements[Air] = 7;
        elements[Fire] = 3;
    }),
    Potion::new("vial of fire", Flame, |elements| {
        elements[Fire] = 8;
        elements[Earth] = 1;
    }),
    // ------ Requires advanced elements ------ //
    Potion::new("protection from charms", CharmProtection, |elements| {
        elements[Fire] = 4;
        elements[Earth] = 4;
        elements[Light] = 2;
        elements[Shadow] = 2;
    }),
    Potion::new("salve of healing", WoundHealing, |elements| {
        elements[Earth] = 4;
        elements[Air] = 4;
        elements[Light] = 4;
    }),
    Potion::new("cough remedy", CoughRemedy, |elements| {
        elements[Ice] = 2;
        elements[Thunder] = 2;
        elements[Air] = 1;
    }),
    Potion::new("fever reducer", FeverReducer, |elements| {
        elements[Ice] = 3;
        elements[Water] = 2;
        elements[Shadow] = 1;
    }),
    Potion::new("insect repellent", InsectRepellent, |elements| {
        elements[Light] = 4;
        elements[Air] = 3;
        elements[Fire] = 2;
    }),
    Potion::new("snake repellent", SnakeRepellent, |elements| {
        elements[Ice] = 3;
        elements[Fire] = 2;
    }),
    Potion::new("potion of cleanliness", Cleanliness, |elements| {
        elements[Void] = 6;
        elements[Air] = 3;
        elements[Light] = 2;
        elements[Ice] = 1;
    }),
    Potion::new("potion of fear", Fear, |elements| {
        elements[Ice] = 3;
        elements[Shadow] = 3;
        elements[Water] = 2;
        elements[Thunder] = 1;
    }),
    Potion::new("potion of rage", Rage, |elements| {
        elements[Fire] = 3;
        elements[Shadow] = 3;
        elements[Taint] = 1;
    }),
    Potion::new("potion of courage", Courage, |elements| {
        elements[Spirit] = 3;
        elements[Fire] = 2;
    }),
    Potion::new("sleep draught", Sleep, |elements| {
        elements[Water] = 3;
        elements[Taint] = 2;
    }),
    Potion::new("paralyzing poison", Paralysis, |elements| {
        elements[Taint] = 3;
        elements[Void] = 3;
    }),
    Potion::new("intelligence potion", Intelligence, |elements| {
        elements[Water] = 5;
        elements[Spirit] = 4;
        elements[Fire] = 3;
    }),
    Potion::new("strength potion", Strength, |elements| {
        elements[Earth] = 6;
        elements[Thunder] = 2;
        elements[Ice] = 1;
    }),
    Potion::new("potion of plant growth", PlantGrowth, |elements| {
        elements[Water] = 4;
        elements[Light] = 4;
        elements[Earth] = 2;
        elements[Air] = 2;
    }),
    Potion::new("potion of resillience", Resillience, |elements| {
        elements[Earth] = 4;
        elements[Shadow] = 2;
    }),
    Potion::new("speed booster", Speed, |elements| {
        elements[Air] = 6;
        elements[Thunder] = 6;
        elements[Light] = 3;
    }),
    Potion::new("charisma", Charisma, |elements| {
        elements[Spirit] = 6;
        elements[Void] = 3;
    }),
    Potion::new("potion of seeing", Perception, |elements| {
        elements[Light] = 5;
        elements[Shadow] = 4;
    }),
    Potion::new("vial of shock", Shock, |elements| {
        elements[Light] = 5;
        elements[Fire] = 4;
        elements[Thunder] = 3;
    }),
    Potion::new("vial of lightning", Lightning, |elements| {
        elements[Thunder] = 6;
        elements[Light] = 6;
    }),
    Potion::new("vial of ice", Freeze, |elements| {
        elements[Ice] = 8;
        elements[Earth] = 1;
    }),
    Potion::new("vial of poison", Poison, |elements| {
        elements[Taint] = 6;
        elements[Air] = 2;
    }),
    Potion::new("vial of darkness", Darkness, |elements| {
        elements[Shadow] = 6;
        elements[Taint] = 1;
        elements[Thunder] = 1;
    }),
));
