use enum_map::Enum;
use serde::{Serialize, Deserialize};

use crate::Solvent;

use Element::*;

pub const TAINTABLE_ELEMENTS: [Element; 11] = [
    Mana,
    Spirit,
    Shadow,
    Void,
    Thunder,
    Air,
    Ice,
    Light,
    Fire,
    Water,
    Earth,
];

pub const EVAPORABLE_ELEMENTS: [Element; 8] = [
    Void,
    Air,
    Spirit,
    Light,
    Fire,
    Shadow,
    Water,
    Mana,
];


#[derive(Clone, Copy, Debug, strum_macros::Display, Enum, PartialEq, Serialize, Deserialize)]
pub enum Element {
    Earth,
    Water,
    Air,
    Fire,
    Spirit, // AKA Ether
    Light,
    Shadow,
    Ice,
    Thunder,
    Mana,
    Taint,
    Void,
}

impl Element {
    pub fn soluble(&self, solvent: &Solvent) -> bool {
        match solvent {
            Solvent::Water => !matches!(self, Earth | Taint | Mana),
            Solvent::Ether => !matches!(self, Earth | Thunder),
            Solvent::Oil => matches!(self, Void | Air | Taint | Light | Shadow),
            Solvent::Air => matches!(self, Void | Air | Spirit | Light),
            Solvent::Vivo => true,
        }
    }

    pub fn warning(&self) -> &'static str {
        match self {
            Water => "Your brew gurgles unnaturally. You think you had better stop adding water.",
            // Water => "Your brew is getting quite close to the rim of the cauldron. You think you had better stop adding water."
            Earth => "Your brew shows signs of thickening and smells earthy.",
            Fire => "The boiling cauldron bubbles with small flashes of blue and orange.",
            Air => "Wisps of steam start dancing in unusual ways.",
            Spirit => "You cauldron takes on a ghostly appearance for a moment, then returns to normal.",
            Light => "The mixture begins glowing unstably.",
            Shadow => "The brew takes on a deep, dark color. It looks concerningly shadowy.",
            Ice => "The fire still roars below the cauldron, yet the mixture turns cool.",
            Thunder => "The cauldron rattles warningly.",
            Mana => "The brew begins fizzing and sparkling.",
            Taint => "As the taint spreads, the mixture turns green and slightly stretchy.",
            Void => "The mixure turns slightly transparent. It almost looks like it's not even there.",
        }
    }

    pub fn unstable_message(&self) -> &'static str {
        // Special effects: Water, air, light, ice, thunder, taint
        // Everything else should empty the cauldron
        match self {
            Water => "A fountain jets up from the middle of your cauldron. A splash over the edge loses you elemental",
            // Water => "With too much water, your cauldron boils over. The mixture spills, losing elemental",
            Earth => "The mixture thickens. You quickly dump out the cauldron and watch as your concoction turns rock solid.",
            Fire => "Your cauldron lights on fire. This is why you do alchemy outdoors. You dump the ash and scrub the cauldron.",
            Air => "A whirlwind appears right inside your cauldron. When it winds down, half of your mixture is gone.",
            Spirit => "Your cauldron turns etherial and fades in and out of existence. Eventually it returns as solid as usual, but completely empty.",
            Light => "The brew turns blindingly bright. You shield your eyes, and when it's over, you see all elemental light has disappeared.",
            Shadow => "You cauldron is engulfed in a thick and creeping darkness. You flee. When you return, faint wisps of shadow still swirl about, but the cauldron is empty.",
            Ice => "The brew crackles and freezes solid. You stoke the fire, but when it finally thaws, all that's left is plain water.",
            Thunder => "The cauldron suddenly rattles. Alarmed, you back up just before a great 'boom' sends your mixture spattering across the yard. Only a little is left of it.",
            Mana => "Suddenly, instead of containing a potion, your cauldron is full of butterflies. They flutter away in a colorful swarm, leaving you with an empty cauldron.",
            Taint => "As the elemental magic becomes tainted, the mixture takes on a slimy appearance and begins creeping up the sides of the cauldron. You run, but not before some of your ingredients become tainted.",
            Void => "With a small 'pop', the mixture in your cauldron suddenly disappears. No trace of your work remains.",
        }
    }

    pub fn base_stability(&self) -> i32 {
        match self {
            Earth => 6,
            Void => 4,
            _ => 5,
        }
    }
}
