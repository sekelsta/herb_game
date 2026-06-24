use std::collections::HashSet;
use enum_map::EnumMap;

use crate::Effect;

pub const ALCHEMY_BOOK: &str = "Alchemy for Dummies
If you are just starting out, alchemy can seem quite daunting! But don't worry, it is.
There are two main procedures to know about: infusion and decoction. Both are much simpler than they sound. Infusion is soaking the ingredient in a liquid for a long time. This is typically done in a jar or bottle, left sitting in a cool dark place such as a shelf. Meanwhile decoction is a shorter soak at a much hotter temperature, that is, your standard boil-in-a-cauldron treatment witches have been practicing for aeons.
Whichever method you use, you will need to concern yourself with the elemental energies of your materials. Each desired effect requires a certain combination of elements to achieve. Some variance is allowable, but makes for a weaker potion.
So how do you actually brew a potion? Start with a liquid base, usually water but you may also use spirits or oil. Bring your cauldron to a boil, then add your herbs one at a time. Each herb you add may provide elemental energy directly, strengthen or weaken pre-existing elements from earlier additions to your brew, or affect the stability of an element. Having a lot of any single element in a potion can lead to unexpected effects. Increased stability allows for more extreme potions, while destabilizing effects can turn your potion into an elemental chain reaction even at weaker strengths. Speaking of chain reactions, watch out for taint. Any elemental taint in your brew will convert other elements into more of itself, which can quickly get out of hand. As you go certain elements may evaporate, so take care with the order and timing. Try out a practice batch and you'll understand. You can dump out your cauldron if the contents have become unsalvageable. Or if you're satisfied with what you've made, bottle it and wash your cauldron for the next batch.
Infusions are if anything simpler than decoctions. Put your herb right into a bottle with any liquid base, and leave it in a cool dark place, perhaps a shelf, while you sleep. After enough time has passed you will see that the liquid has taken on some of the color of the plant. This indicates that the plant's elemental energies have leached into the liquid, and your infusion is ready to be filtered and put to use. You can make potions directly as infusions, you can add them to your boiling decoctions in place of the herb, or you can even add another herb and infuse again. You can also infuse an herb into a decoction, with or without existing potion effects, to add to the elements and potentially create a new potion out of it. Note that infusing an herb and decocting an herb will not have quite the same effect. Only elements the herb provides directly will be available to create potion effects on the infusion. Herbs which strengthen existing elements will not do so when added to an infusion. Another thing to note is that more of the elemental energy stays with the plant compared to a decoction. Certain types of elements, if they are not soluble in your chosen base, will not become available at all. This can be used to your advantage to purify the remaining elements, and can allow for higher quality potions if you know what you're doing.
That's about all you need to know to get started. I encourage you to experiment for youself to discover what effects you can create. With trial and error you'll be able to refine your recipes, and as you go on you'll discover cheaper and more effective combinations of ingredients. Good luck!";

pub struct KnowledgeState {
    pub herb_tier: i32,
    pub effects: EnumMap<Effect, bool>,
    pub herb_species: HashSet<&'static str>,
    pub herbs_gathered: u32,
    pub stability_known: bool,

    pub max_tier: i32,
    // To next level
    pub next_effects: usize,
    pub next_species: usize,
    pub next_gathered: u32,
}

impl KnowledgeState {
    pub fn new() -> Self {
        KnowledgeState {
            herb_tier: 0,
            effects: EnumMap::default(),
            herb_species: HashSet::new(),
            herbs_gathered: 0,
            stability_known: false,

            max_tier: 3,
            next_effects: 2,
            next_species: 5,
            next_gathered: 12,
        }
    }

    pub fn count_effects(&self) -> usize {
        self.effects.values().filter(|x| **x).count()
    }

    pub fn ready_to_advance(&self) -> bool {
        self.herbs_gathered >= self.next_gathered && self.herb_species.len() >= self.next_species && self.count_effects() >= self.next_effects
    }

    pub fn update(&mut self) -> Option<String> {
        if !self.ready_to_advance() {
            return None;
        }
        self.herb_tier += 1;
        match self.herb_tier {
            1 => {
                self.next_effects = 5;
                self.next_species = 8;
                self.next_gathered = 24;
                // TODO: Add custom per-tier suggestions on where to explore next
                Some("You've learned to recognize new plant species!".to_string())
            },
            2 => {
                self.next_effects = 8;
                self.next_species = 12;
                self.next_gathered = 32;
                Some("You've learned to recognize new plant species!".to_string())
            },
            // Max level
            x if x >= self.max_tier => Some("You've learned to recognize new plant species!".to_string()),
            // Shouldn't happen, but don't crash if it does
            _ => Some("Bug the developer to fix herb tiers.".to_string()),
        }
    }

    pub fn status(&self) -> String {
        let effects = self.count_effects();
        let species = self.herb_species.len();
        let gathered = self.herbs_gathered;
        if self.herb_tier >= self.max_tier {
            return "You've learned everything. Good job!".to_string();
        }
        if effects == 0 {
            if gathered == 0 {
                return "You haven't started yet.".to_string();
            }
            return format!("Gathered {}/{} herbs of {}/{} species. No potions brewed.", gathered, self.next_gathered, species, self.next_species);
        }
        if self.ready_to_advance() {
            return "You've had a long day. Try sleeping on it.".to_string();
        }
        format!("Gathered {}/{} herbs of {}/{} species and brewed potions with {}/{} unique effects.", gathered, self.next_gathered, species, self.next_species, effects, self.next_effects)
    }
}
