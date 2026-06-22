use once_cell::sync::Lazy;

use crate::Ingredient;
use crate::Element::*;
use crate::Modifier::*;

pub static DANDELION: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("dandelion", 0.0, |elements| {
    elements[Air][Provide] = 2;
    elements[Fire][Provide] = 1;
    elements[Mana][Provide] = 1;
    elements[Water][Stabilize] = -1;
}));

pub static VIOLET: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("violet", 0.0, |elements| {
    elements[Shadow][Provide] = 2;
    elements[Shadow][Strengthen] = 1;
}));

pub static BLUEBELL: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("bluebell", 0.2, |elements| {
    elements[Mana][Provide] = 2;
    elements[Void][Provide] = 2;
    elements[Taint][Provide] = 1;
}));

pub static WATERMINT: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("watermint", 0.0, |elements| {
    elements[Ice][Provide] = 3;
    elements[Shadow][Provide] = 2;
    elements[Water][Provide] = 1;
}));

pub static WINTERGREEN: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("wintergreen", 0.0, |elements| {
    elements[Ice][Provide] = 4;
    elements[Earth][Provide] = 1;
    elements[Earth][Stabilize] = 2;
}));

pub static JEWELWEED: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("jewelweed", 0.1, |elements| {
    elements[Ice][Provide] = 2;
    elements[Mana][Provide] = 2;
    elements[Air][Stabilize] = 3;
    elements[Thunder][Stabilize] = -1;
}));

pub static PETTY_SPURGE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("petty spurge", 0.3, |elements| {
    elements[Shadow][Provide] = 1;
    elements[Taint][Provide] = 1;
    elements[Ice][Strengthen] = 4;
}));

pub static PURSLANE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("purslane", 0.0, |elements| {
    elements[Water][Provide] = 2;
    elements[Ice][Provide] = 1;
}));

pub static MILKWEED: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("milkweed", 0.1, |elements| {
    elements[Ice][Strengthen] = 2;
}));

pub static FEVERFEW: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("feverfew", 0.0, |elements| {
    elements[Ice][Provide] = 1;
}));

pub static COLTSFOOT: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("coltsfoot", 0.01, |elements| {
    elements[Thunder][Provide] = 2;
}));

pub static HORSETAIL: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("horsetail", 0.01, |elements| {
    elements[Thunder][Provide] = 1;
    elements[Thunder][Strengthen] = 3;
}));

pub static COLUMBINE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("columbine", 0.1, |elements| {
    elements[Thunder][Provide] = 1;
}));

pub static ENCHANTERS_NIGHTSHADE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("enchanter's nightshade", 0.05, |elements| {
    elements[Thunder][Stabilize] = 4;
}));

pub static SPOTTED_DEADNETTLE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("spotted deadnettle", 0.05, |elements| {
    elements[Shadow][Provide] = 3;
    elements[Earth][Provide] = 2;
    elements[Void][Stabilize] = 2;
}));

pub static SKUNK_CABBAGE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("skunk cabbage", 0.6, |elements| {
    elements[Fire][Provide] = 2;
    elements[Fire][Strengthen] = 2;
    elements[Fire][Stabilize] = -2;
    elements[Taint][Provide] = 1;
    elements[Taint][Stabilize] = 2;
    elements[Earth][Strengthen] = 3;
    elements[Spirit][Provide] = 1;
}));

pub static WHITE_CLOVER: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("white clover", 0.0, |elements| {
    elements[Mana][Provide] = 1;
    elements[Shadow][Strengthen] = -2;
}));

pub static RED_CLOVER: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("red clover", 0.0, |elements| {
    elements[Fire][Provide] = 1;
    elements[Light][Strengthen] = 2;
}));

pub static DAFFODIL: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("daffodil", 0.2, |elements| {
    elements[Light][Provide] = 3;
}));

pub static BUTTERCUP: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("buttercup", 0.2, |elements| {
    elements[Fire][Provide] = 1;
    elements[Light][Strengthen] = 2;
}));

pub static OXEYE_DAISY: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("ox-eye daisy", 0.0, |elements| {
    elements[Light][Provide] = 1;
    elements[Light][Stabilize] = 3;
    elements[Shadow][Stabilize] = -3;
}));

pub static BULL_THISTLE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("bull thistle", 0.2, |elements| {
    elements[Thunder][Provide] = 2;
    elements[Thunder][Strengthen] = 2;
    elements[Thunder][Stabilize] = -1;
    elements[Fire][Provide] = 1;
    elements[Fire][Strengthen] = 3;
    elements[Shadow][Stabilize] = -2;
    elements[Mana][Stabilize] = -2;
}));

pub static FOX_SEDGE: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("fox sedge", 0.0, |elements| {
    elements[Water][Strengthen] = 3;
    elements[Shadow][Strengthen] = 2;
    elements[Mana][Strengthen] = -1;
    elements[Void][Stabilize] = 3;
}));

pub static BURDOCK: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("burdock", 0.0, |elements| {
    elements[Earth][Provide] = 3;
    elements[Mana][Provide] = 2;
    elements[Thunder][Provide] = 1;
    elements[Fire][Stabilize] = 3;
    elements[Ice][Stabilize] = -3;
}));

pub static HEALALL: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("heal-all", 0.0, |elements| {
    elements[Earth][Provide] = 2;
    elements[Water][Provide] = 1;
    elements[Thunder][Provide] = 1;
    elements[Light][Strengthen] = 3;
    elements[Shadow][Provide] = 1;
    elements[Shadow][Stabilize] = 1;
}));

pub static YARROW: Lazy<Ingredient> = Lazy::new(|| Ingredient::new_herb("heal-all", 0.0, |elements| {
    elements[Air][Provide] = 4;
    elements[Earth][Provide] = 2;
}));

// Comfrey, boneset
