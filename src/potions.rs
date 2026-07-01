use once_cell::sync::Lazy;
use enum_map::{Enum, EnumMap};
use serde::{Serialize, Deserialize};
use strum_macros::{Display, IntoStaticStr};

use crate::{Element, Ingredient, Modifier, Solvent};
use Element::*;
use Effect::*;

#[derive(Clone, Copy, Debug, Display, IntoStaticStr, Enum, Eq, PartialEq, Serialize, Deserialize)]
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
    Resilience,
    Speed,
    Charisma,
    Perception,
    Loveliness,
    Cleanliness,
    Flame, 
    Lightning,
    Freeze,
    Shock,
    Shine,
    Poison,
    Darkness,
    Butterflies,
    Fireflies,
    Invisibility,
    Feysight,
    Alertness,
    Sparkles,
}

impl Effect {
    pub fn to_title_case(&self) -> &'static str {
        match self {
            HealthBoost => "Health Boost",
            CoughRemedy => "Cough Remedy",
            FeverReducer => "Fever Reducer",
            InsectRepellent => "Insect Repellent",
            SnakeRepellent => "Snake Repellent",
            CharmProtection => "Charm Protection",
            PlantGrowth => "Plant Growth",
            WoundHealing => "Wound Healing",
            e => e.into(),
        }
    }

    pub fn sale_value(&self) -> i32 {
        match self {
            HealthBoost => 6,
            CoughRemedy => 8,
            FeverReducer => 8,
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
            Resilience => 15,
            Speed => 14,
            Charisma => 18,
            Perception => 15,
            Loveliness => 22,
            Cleanliness => 18,
            Flame => 24, 
            Lightning => 25,
            Freeze => 26,
            Shock => 23,
            Shine => 15,
            Poison => 13,
            Darkness => 28,
            Butterflies => 32,
            Fireflies => 30,
            Sparkles => 4,
            Invisibility => 36,
            Feysight => 12,
            Alertness => 5,
        }
    }

    pub fn potion_name(&self, solvent: &Solvent) -> String {
        match self {
            HealthBoost => "health tonic".to_string(),
            Relaxation => "relaxant".to_string(),
            CharmProtection => "protection from charms".to_string(),
            Loveliness => "perfume of loveliness".to_string(),
            Paralysis => "paralyzing poison".to_string(),
            Perception => "potion of seeing".to_string(),
            WoundHealing => "salve of healing".to_string(),
            Freeze => "potion of ice".to_string(),
            Shine => "liquid starshine".to_string(),
            Sparkles => "sparkling potion".to_string(),
            CoughRemedy | FeverReducer | InsectRepellent | SnakeRepellent | Charisma | Shock | Lightning | Poison | Darkness | Flame =>
                self.to_title_case().to_ascii_lowercase(),
            Patience | Rage | Fear | Courage | Resilience | Cleanliness | PlantGrowth | Butterflies | Fireflies | Feysight | Alertness =>
                format!("potion of {}", self.to_title_case().to_ascii_lowercase()),
            Love | Strength | Intelligence | Invisibility =>
                format!("{} potion", self.to_title_case().to_ascii_lowercase()),
            Sleep =>
                format!("{} draught", self.to_title_case().to_ascii_lowercase()),
            Speed =>
                format!("{} booster", self.to_title_case().to_ascii_lowercase()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Potion {
    pub effect: Effect,
    pub elements: EnumMap<Element, i32>,
    //pub min_toxicity: f32,
    //pub max_toxicity: f32,
}

impl Potion {
    pub fn new(effect: Effect, f: impl Fn(&mut EnumMap<Element, i32>)) -> Self {
        let mut elements: EnumMap<Element, i32> = EnumMap::default();
        f(&mut elements);
        Self {
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
    // ------ Tier 0 ------ //
    Potion::new(HealthBoost, |elements| {
        elements[Earth] = 3;
        elements[Air] = 3;
    }),
    Potion::new(Relaxation, |elements| {
        elements[Water] = 3;
        elements[Fire] = 2;
        elements[Earth] = 1;
        elements[Light] = 1;
        elements[Spirit] = 1;
    }),
    Potion::new(Patience, |elements| {
        elements[Earth] = 6;
        elements[Water] = 4;
    }),
    Potion::new(Alertness, |elements| {
        elements[Fire] = 3;
        elements[Earth] = 2;
    }),
    // ------ Tier 1 ------ //
    Potion::new(Sleep, |elements| {
        elements[Water] = 3;
        elements[Shadow] = 3;
        elements[Spirit] = 1;
        elements[Taint] = 1;
    }),
    Potion::new(Love, |elements| {
        elements[Fire] = 3;
        elements[Air] = 3;
        elements[Light] = 3;
        elements[Ice] = 1;
        elements[Thunder] = 1;
    }),
    Potion::new(Sparkles, |elements| {
        elements[Light] = 3;
        elements[Fire] = 1;
        elements[Air] = 1;
    }),
    Potion::new(CharmProtection, |elements| {
        elements[Fire] = 4;
        elements[Earth] = 4;
        elements[Light] = 2;
        elements[Shadow] = 2;
    }),
    Potion::new(WoundHealing, |elements| {
        elements[Earth] = 4;
        elements[Air] = 4;
        elements[Light] = 4;
    }),
    Potion::new(InsectRepellent, |elements| {
        elements[Light] = 4;
        elements[Air] = 3;
        elements[Fire] = 2;
    }),
    Potion::new(Intelligence, |elements| {
        elements[Water] = 5;
        elements[Spirit] = 4;
        elements[Fire] = 3;
    }),
    Potion::new(PlantGrowth, |elements| {
        elements[Water] = 4;
        elements[Light] = 4;
        elements[Earth] = 2;
        elements[Air] = 2;
    }),
    Potion::new(Resilience, |elements| {
        elements[Earth] = 4;
        elements[Shadow] = 2;
    }),
    Potion::new(Courage, |elements| {
        elements[Spirit] = 3;
        elements[Fire] = 2;
    }),
    Potion::new(Perception, |elements| {
        elements[Light] = 5;
        elements[Shadow] = 4;
    }),
    Potion::new(Darkness, |elements| {
        elements[Shadow] = 6;
        elements[Taint] = 1;
        elements[Thunder] = 1;
    }),
    Potion::new(Invisibility, |elements| {
        elements[Shadow] = 8;
        elements[Air] = 4;
        elements[Light] = 3;
        elements[Mana] = 2;
    }),
    // ------ Requires advanced elements ------ //
    Potion::new(Feysight, |elements| {
        elements[Mana] = 4;
        elements[Light] = 2;
        elements[Air] = 2;
        elements[Shadow] = 1;
    }),
    Potion::new(Shine, |elements| {
        elements[Light] = 4;
        elements[Mana] = 2;
    }),
    Potion::new(Butterflies, |elements| {
        elements[Mana] = 6;
        elements[Air] = 5;
        elements[Light] = 2;
    }),
    Potion::new(Fireflies, |elements| {
        elements[Mana] = 6;
        elements[Light] = 5;
        elements[Air] = 2;
    }),
    Potion::new(Loveliness, |elements| {
        elements[Air] = 7;
        elements[Fire] = 3;
    }),
    Potion::new(Flame, |elements| {
        elements[Fire] = 8;
        elements[Mana] = 2;
        elements[Earth] = 1;
    }),
    Potion::new(CoughRemedy, |elements| {
        elements[Ice] = 2;
        elements[Thunder] = 2;
        elements[Air] = 1;
    }),
    Potion::new(FeverReducer, |elements| {
        elements[Ice] = 3;
        elements[Water] = 2;
        elements[Shadow] = 1;
    }),
    Potion::new(SnakeRepellent, |elements| {
        elements[Ice] = 3;
        elements[Fire] = 2;
    }),
    Potion::new(Cleanliness, |elements| {
        elements[Void] = 6;
        elements[Air] = 3;
        elements[Light] = 2;
        elements[Ice] = 1;
    }),
    Potion::new(Fear, |elements| {
        elements[Ice] = 3;
        elements[Shadow] = 3;
        elements[Water] = 2;
        elements[Thunder] = 1;
    }),
    Potion::new(Rage, |elements| {
        elements[Fire] = 3;
        elements[Shadow] = 3;
        elements[Taint] = 1;
    }),
    Potion::new(Paralysis, |elements| {
        elements[Taint] = 3;
        elements[Void] = 3;
    }),
    Potion::new(Strength, |elements| {
        elements[Earth] = 6;
        elements[Thunder] = 2;
        elements[Ice] = 1;
    }),
    Potion::new(Speed, |elements| {
        elements[Air] = 6;
        elements[Thunder] = 6;
        elements[Light] = 3;
    }),
    Potion::new(Charisma, |elements| {
        elements[Spirit] = 6;
        elements[Void] = 3;
    }),
    Potion::new(Shock, |elements| {
        elements[Light] = 5;
        elements[Fire] = 4;
        elements[Thunder] = 3;
        elements[Mana] = 2;
    }),
    Potion::new(Lightning, |elements| {
        elements[Thunder] = 6;
        elements[Light] = 6;
        elements[Mana] = 2;
    }),
    Potion::new(Freeze, |elements| {
        elements[Ice] = 8;
        elements[Mana] = 2;
        elements[Earth] = 1;
    }),
    Potion::new(Poison, |elements| {
        elements[Taint] = 6;
        elements[Air] = 2;
    }),
));
