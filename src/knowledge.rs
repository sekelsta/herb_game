use std::collections::{HashMap, HashSet};
use enum_map::EnumMap;
use serde::{Serialize, Deserialize};

use crate::{Effect, Element, Plant, Ingredient, Modifier, RegionEnum};
use crate::potions::REFERENCE_POTIONS;
use crate::herbs::REFERENCE_HERBS;

pub const ALCHEMY_BOOK_INTRO: &str = "Introduction to Herbal Brews
Ever wonder what is happening when you boil an herb in your cauldron? There is the obvious change you see, which is the herb wilting and the water taking its color. But there are also elemental energies at play. Boiling in water allows those energies to be released into the water, where they become available for the mystical effects of potions. Adding another herb will release its elements as well, but as you wait, the lighter elements will evaporate. You can stir the cauldron in the rare case where you want evaporation to happen faster. If you want to take a pause, you can bottle your mixture and add it back later.
Which, if any, potion is created depends on how many energies of each element are available in the brew. For instance, a combination of air and earth without much fire is good for healing, as written by Guldin Schatzkarmmer in 1598. Or for relaxation, use water warmed by fire, with hints of something more. The potion will be weaker if the elements available are not quite the right ones. I recommend experimenting with combinations of several herbs to see what effects you can discover.
Oh, and be aware, making powerful potions often requires a great deal of elemental energies, yet taking that too far can have ...exciting... results. Be sure to set up your workspace outside and in an area cleared of flammable materials.";

pub const ALCHEMY_BOOK_STABILITY: &str = "Experiments on Concentrated Earth
Trial 1: Start with an empty cauldron. Add water and bring it to a boil. Then add three dandelions, one at a time. Notice a change in the texture of the brew as elemental earth is extracted. Add a fourth dandelion and quickly dump the mixture. Watch as it solidifies into rock.
Trial 2: Start again with an empty cauldron, and again add water and bring it to a boil. Add three dandelions and notice the texture change. Add red clover and notice the texture change back. Then add a fourth dandelion, and notice that though it thickens again, the earth stays stable in the liquid brew. A touch of water and spirit, and I ended up with a nice stoneskin potion.
Trial 3: As before, but red clover was added first, then dandelions. The mixture began to thicken at four dandelions, but held its liquid form.
What does this mean? It seems the red clover is having some effect on the earth provided by the dandelion. Further experimentation may clarify the details.";

pub const ALCHEMY_BOOK_REACTION: &str = "Dear friend,
I hope this finds you well. As you can see my raven is doing much better now. Thank you again for looking after him when he fell ill last year.
I've been studying everything I could find about healing methods since then, and had an interesting result I thought you'd like to hear about.
I brewed a health tonic and then experimented with adding further herbs. I tried adding first sunflower, and then red clover, and found a more potent restorer of health. The second batch I added the red clover first and then the sunflower, but the effect was not as strong in the end.
It seems something in the red clover is reacting with the light of the sunflower. I'll write this down as 0 (+2) Light, because it provides 0 light directly, but up to 2 if that much is already present.
How have you been? I hear you opened a shop? Please write and tell me all about it!
Sincerely, Sharon";

pub const ALCHEMY_BOOK_INFUSIONS: &str = "Infusion of herbs
Usually, we boil an herb in water to extract the elemental energies. However, I've found that soaking it in cold water overnight also works. You can also use spirits or oil as a base instead of water. Leave the herb to soak in a cool, dark, place, such as a shelf, and once it's ready you'll see the liquid has taken on the color and elemental properties of the herb. Unlike a boiled decoction, subtle effects are preserved. Another thing to note is that more of the elemental energy stays with the plant compared to a decoction. And certain types of elements, if they are not soluble in your chosen base, will not become available at all. This can be used to your advantage to purify the remaining elements, and can allow for higher quality potions if you know what you're doing.
For a stronger infusion, you can add another herb the next day. You can also infuse into a tea, or brew your infusion in a cauldron.";

pub const MAX_TIER: i32 = 4;

#[derive(Serialize, Deserialize)]
pub struct KnowledgeState {
    pub herb_tier: i32,
    recipes: EnumMap<Effect, Vec<(f32, EnumMap<Element, i32>)>>,
    pub herb_locations: HashMap<Plant, HashSet<RegionEnum>>,
    pub herbs_gathered: u32,
    pub known_elements: HashMap<Plant, EnumMap<Element, EnumMap<Modifier, bool>>>,
}

impl KnowledgeState {
    pub fn new() -> Self {
        KnowledgeState {
            herb_tier: 0,
            recipes: EnumMap::default(),
            herb_locations: HashMap::new(),
            herbs_gathered: 0,
            known_elements: HashMap::new(),
        }
    }

    pub fn mark_herb_found(&mut self, species: Plant, region: RegionEnum) {
        let set = self.herb_locations.entry(species).or_insert(HashSet::default());
        set.insert(region);
    }

    pub fn mark_recipe(&mut self, ingredient: &Ingredient) {
        if let Some(effect) = ingredient.effect {
            let mut elements = EnumMap::default();
            for (element, modifiers) in &ingredient.elements {
                elements[element] = modifiers[Modifier::Provide];
            }
            for (_strength, recipe) in &self.recipes[effect] {
                if *recipe == elements {
                    return;
                }
            }
            self.recipes[effect].push((ingredient.strength, elements));
            // Sort descending
            self.recipes[effect].sort_by(|(strength_a, _), (strength_b, _)| strength_b.total_cmp(strength_a));
        }
    }

    pub fn count_effects(&self) -> usize {
        self.recipes.values().filter(|x| x.len() > 0).count()
    }

    pub fn wine_unlocked(&self) -> bool {
        self.herb_tier >= 1
    }

    pub fn spirits_unlocked(&self) -> bool {
        self.herb_tier >= 2
    }

    pub fn oil_unlocked(&self) -> bool {
        self.herb_tier >= 3
    }

    pub fn stability_known(&self) -> bool {
        self.herb_tier >= 1
    }

    pub fn infusion_known(&self) -> bool {
        self.herb_tier >= 3
    }

    pub fn knows_element(&self, element: Element) -> bool {
        use Element::*;
        match element {
            Earth | Water | Fire | Air => true,
            Spirit | Light | Shadow => self.herb_tier >= 1,
            Ice | Thunder | Mana => self.herb_tier >= 2,
            Void | Taint => self.herb_tier >= 3,
        }
    }

    pub fn effects_to_next_level(&self) -> usize {
        match self.herb_tier {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 10,
            _ => REFERENCE_POTIONS.len(),
        }
    }

    pub fn species_to_next_level(&self) -> usize {
        match self.herb_tier {
            0 => 5,
            1 => 10,
            2 => 15,
            3 => 20,
            _ => REFERENCE_HERBS.len(),
        }
    }

    pub fn ready_to_advance(&self) -> bool {
        self.herb_locations.len() >= self.species_to_next_level() && self.count_effects() >= self.effects_to_next_level()
    }

    pub fn update(&mut self) -> Option<String> {
        if !self.ready_to_advance() {
            return None;
        }
        self.herb_tier += 1;
        match self.herb_tier {
            1 => Some("You had a dream about studying plants with your grandma. In the morning, you find a note that definitely wasn't there before. Type 'note' to read it.\nYou feel confident about more plant species and also ready to try something new from the village market.".to_string()),
            2 => Some("A raven flies in through the window, with a piece of paper tied to its leg. It stands still while you take it, then flies off. Type 'letter' to read it.\nYou've also learned about new species of plants and new ingredients from the market.".to_string()),
            // TODO: Consider requiring a certain level of potion strength as well
            3 => Some("You feel comfortable with your cauldron and ready to read about new methods. Type 'infusions' to see what the library has on it. You've also learned to recognize new herbs and have an idea for something you saw at the market.".to_string()),
            // TODO: Distillation here
            4 => Some("You've become a well-known alchemist!".to_string()),
            // Max level
            _ => None,
        }
    }

    pub fn status(&self) -> String {
        let effects = self.count_effects();
        let species = self.herb_locations.len();
        let gathered = self.herbs_gathered;
        if self.herb_tier > MAX_TIER {
            return "You've learned everything. Good job!".to_string();
        }
        if effects == 0 {
            if gathered == 0 {
                return "You haven't started yet.".to_string();
            }
            return format!("Gathered {} herbs of {}/{} species. No potions brewed.", gathered, species, self.species_to_next_level());
        }
        if self.ready_to_advance() {
            return "You've had a long day. Try sleeping on it.".to_string();
        }
        format!("Gathered {} herbs of {}/{} species and brewed potions with {}/{} unique effects.", gathered, species, self.species_to_next_level(), effects, self.effects_to_next_level())
    }

    pub fn list_herb_locations(&self) -> String {
        if self.herb_locations.is_empty() {
            return "You haven't collected any herbs yet, but you spotted some you recognized in the overgrown farm field north of town.".to_string();
        }
        self.herb_locations.iter().map(|(herb, locations)| format!("{}: {}", herb.to_lowercase_string(), locations.iter().map(|x| x.to_title_case().to_ascii_lowercase()).collect::<Vec<String>>().join(", "))).collect::<Vec<String>>().join("\n")
    }

    pub fn list_recipes(&self) -> String {
        let mut all_recipes = Vec::new();
        for (effect, potions) in &self.recipes {
            for (strength, elements) in potions {
                let data = elements.iter().filter(|(_e, x)| **x != 0).map(|(e, x)| format!("{} {:?}", x, e)).collect::<Vec<String>>().join(", ");
                all_recipes.push(format!("{} ({}% strength): {}", effect.to_title_case(), (strength * 100.0).round() as i32, data));
            }
        }
        if all_recipes.is_empty() {
            return "Try brewing something first.".to_string();
        }
        all_recipes.join("\n")
    }

    pub fn book(&self) -> String {
        ALCHEMY_BOOK_INTRO.to_string()
    }

    pub fn show_stability_note(&self) -> String {
        if self.herb_tier < 1 {
            return "You feel like you could do with more hands-on experience before reading theory.".to_string();
        }

        ALCHEMY_BOOK_STABILITY.to_string()
    }

    pub fn show_boost_note(&self) -> String {
        if self.herb_tier < 2 {
            return "You feel like you could do with more hands-on experience before reading theory.".to_string();
        }

        ALCHEMY_BOOK_REACTION.to_string()
    }

    pub fn show_infusion_instructions(&self) -> String {
        if self.herb_tier < 2 {
            return "You feel like you could do with more hands-on experience before reading theory.".to_string();
        }

        ALCHEMY_BOOK_INFUSIONS.to_string()
    }
}
