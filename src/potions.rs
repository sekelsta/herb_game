use once_cell::sync::Lazy;
use enum_map::Enum;

use crate::{Element, Ingredient};
use Element::*;
use Effect::*;

#[derive(Clone, Copy, Debug, strum_macros::Display, Enum, PartialEq)]
pub enum Effect {
    CoughRemedy,
    FeverReducer,
    InsectRepellent,
    SnakeRepellent,
    CharmProtection,
    PlantGrowth,
    WoundHealing,
    HealthBoost,
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
            CoughRemedy => "Cough Remedy".to_string(),
            FeverReducer => "Fever Reducer".to_string(),
            InsectRepellent => "Insect Repellent".to_string(),
            SnakeRepellent => "Snake Repellent".to_string(),
            CharmProtection => "Charm Protection".to_string(),
            PlantGrowth => "Plant Growth".to_string(),
            WoundHealing => "Wound Healing".to_string(),
            HealthBoost => "Health Boost".to_string(),
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

pub static REFERENCE_POTIONS: Lazy<Vec<Ingredient>> = Lazy::new(|| vec!(
    // ------ Craftable with only basic elements ------ //
    Ingredient::new_potion("health tonic", HealthBoost, 1.0, |elements| {
        elements[Earth] = 4;
        elements[Air] = 4;
    }),
    Ingredient::new_potion("love potion", Love, 1.0, |elements| {
        elements[Fire] = 3;
        elements[Air] = 3;
        elements[Ice] = 1;
        elements[Thunder] = 1;
    }),
    Ingredient::new_potion("relaxant", Relaxation, 1.0, |elements| {
        elements[Water] = 3;
        elements[Fire] = 2;
        elements[Earth] = 1;
        elements[Light] = 1;
        elements[Spirit] = 1;
    }),
    Ingredient::new_potion("potion of patience", Patience, 1.0, |elements| {
        elements[Earth] = 5;
        elements[Water] = 3;
    }),
    Ingredient::new_potion("perfume of loveliness", Loveliness, 1.0, |elements| {
        elements[Air] = 7;
        elements[Fire] = 3;
    }),
    Ingredient::new_potion("vial of fire", Flame, 1.0, |elements| {
        elements[Fire] = 8;
        elements[Earth] = 1;
    }),
    // ------ Requires advanced elements ------ //
    Ingredient::new_potion("protection from charms", CharmProtection, 1.0, |elements| {
        elements[Fire] = 4;
        elements[Earth] = 4;
        elements[Light] = 2;
        elements[Shadow] = 2;
    }),
    Ingredient::new_potion("salve of healing", WoundHealing, 1.0, |elements| {
        elements[Earth] = 4;
        elements[Air] = 4;
        elements[Light] = 4;
    }),
    Ingredient::new_potion("cough remedy", CoughRemedy, 1.0, |elements| {
        elements[Ice] = 2;
        elements[Thunder] = 2;
        elements[Air] = 1;
    }),
    Ingredient::new_potion("fever reducer", FeverReducer, 1.0, |elements| {
        elements[Ice] = 3;
        elements[Water] = 2;
        elements[Shadow] = 1;
    }),
    Ingredient::new_potion("insect repellent", InsectRepellent, 1.0, |elements| {
        elements[Light] = 4;
        elements[Air] = 3;
        elements[Fire] = 2;
    }),
    Ingredient::new_potion("snake repellent", SnakeRepellent, 1.0, |elements| {
        elements[Ice] = 3;
        elements[Fire] = 2;
    }),
    Ingredient::new_potion("potion of cleanliness", Cleanliness, 1.0, |elements| {
        elements[Void] = 6;
        elements[Air] = 3;
        elements[Light] = 2;
        elements[Ice] = 1;
    }),
    Ingredient::new_potion("potion of fear", Fear, 1.0, |elements| {
        elements[Ice] = 3;
        elements[Shadow] = 3;
        elements[Water] = 2;
        elements[Thunder] = 1;
    }),
    Ingredient::new_potion("potion of rage", Rage, 1.0, |elements| {
        elements[Fire] = 3;
        elements[Shadow] = 3;
        elements[Taint] = 1;
    }),
    Ingredient::new_potion("potion of courage", Courage, 1.0, |elements| {
        elements[Spirit] = 3;
        elements[Fire] = 2;
    }),
    Ingredient::new_potion("sleep draught", Sleep, 1.0, |elements| {
        elements[Water] = 3;
        elements[Taint] = 2;
    }),
    Ingredient::new_potion("paralyzing poison", Paralysis, 1.0, |elements| {
        elements[Taint] = 3;
        elements[Void] = 3;
    }),
    Ingredient::new_potion("intelligence potion", Intelligence, 1.0, |elements| {
        elements[Water] = 5;
        elements[Spirit] = 4;
        elements[Fire] = 3;
    }),
    Ingredient::new_potion("strength potion", Strength, 1.0, |elements| {
        elements[Earth] = 6;
        elements[Thunder] = 2;
        elements[Ice] = 1;
    }),
    Ingredient::new_potion("potion of plant growth", PlantGrowth, 1.0, |elements| {
        elements[Water] = 4;
        elements[Light] = 4;
        elements[Earth] = 2;
        elements[Air] = 2;
    }),
    Ingredient::new_potion("potion of resillience", Resillience, 1.0, |elements| {
        elements[Earth] = 4;
        elements[Shadow] = 2;
    }),
    Ingredient::new_potion("speed booster", Speed, 1.0, |elements| {
        elements[Air] = 6;
        elements[Thunder] = 6;
        elements[Light] = 3;
    }),
    Ingredient::new_potion("charisma", Charisma, 1.0, |elements| {
        elements[Spirit] = 6;
        elements[Void] = 3;
    }),
    Ingredient::new_potion("potion of seeing", Perception, 1.0, |elements| {
        elements[Light] = 5;
        elements[Shadow] = 4;
    }),
    Ingredient::new_potion("vial of shock", Shock, 1.0, |elements| {
        elements[Light] = 5;
        elements[Fire] = 4;
        elements[Thunder] = 3;
    }),
    Ingredient::new_potion("vial of lightning", Lightning, 1.0, |elements| {
        elements[Thunder] = 6;
        elements[Light] = 6;
    }),
    Ingredient::new_potion("vial of ice", Freeze, 1.0, |elements| {
        elements[Ice] = 8;
        elements[Earth] = 1;
    }),
    Ingredient::new_potion("vial of poison", Poison, 1.0, |elements| {
        elements[Taint] = 6;
        elements[Air] = 2;
    }),
    Ingredient::new_potion("vial of darkness", Darkness, 1.0, |elements| {
        elements[Shadow] = 6;
        elements[Taint] = 1;
        elements[Thunder] = 1;
    }),
));
