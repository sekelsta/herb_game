use enum_map::EnumMap;
use rand::RngExt;
use serde::{Serialize, Deserialize};

use crate::*;

static UNLIMITED_INGREDIENTS: Lazy<Vec<&'static Ingredient>> = Lazy::new(|| vec!(&*WATER));

#[derive(Serialize, Deserialize)]
pub struct World {
    #[serde(serialize_with = "region::serialize_regions", deserialize_with = "region::deserialize_regions")]
    pub regions: EnumMap<RegionEnum, Region>,
    pub current_region: RegionEnum,
    pub satchel: Vec<Ingredient>,
    pub empty_bottles: i32,
    pub bottles_sold: i32,
    pub money: i32,
    pub infusion_shelf: Vec<Ingredient>,
    pub cauldron: Option<Ingredient>,
    pub discoveries: KnowledgeState,
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            regions: Region::new_regions(),
            current_region: RegionEnum::Hut,
            empty_bottles: 4,
            bottles_sold: 0,
            money: 2,
            satchel: Vec::new(),
            infusion_shelf: Vec::new(),
            cauldron: None,
            discoveries: KnowledgeState::new(),
        };
        world.advance_time();
        world
    }

    pub fn travel_cardinal(&mut self, direction: Direction) -> String {
        let current = self.current_region;
        self.current_region = self.regions[self.current_region].routes[direction];
        return if current == self.current_region {
            "You decide not to travel so far from home after all.".to_string()
        } else {
            self.look()
        }
    }

    pub fn travel_to(&mut self, region: RegionEnum) -> String {
        if region == self.current_region {
            "You're already here.".to_string()
        } else if self.regions[self.current_region].routes.values().any(|r| *r == region) {
            self.current_region = region;
            self.look()
        } else {
            "That's not nearby.".to_string()
        }
    }

    fn has_cauldron(&self) -> bool {
        return self.current_region == RegionEnum::Hut;
    }

    pub fn list_inventory(&self) -> String {
        let satchel_contents = self.list_satchel();
        if satchel_contents == "" {
            format!("Unlimited water\nEmpty glass bottles: {}\nSilver pieces: {}", self.empty_bottles, self.money)
        } else {
            format!("{0}\nUnlimited water\nEmpty glass bottles: {1}\nSilver pieces: {2}", satchel_contents, self.empty_bottles, self.money)
        }
    }

    fn list_satchel(&self) -> String {
        self.satchel.iter().map(|i| i.inventory_view(&self.discoveries)).collect::<Vec<String>>().join("\n")
    }

    pub fn forage(&mut self, count: i32) -> String {
        if REFERENCE_HERBS.values().all(|h| !h.biomes.contains(&self.current_region)) {
            return "Nothing grows here.".to_string();
        }
        let available = &mut self.regions[self.current_region].current_herbs;
        if available.len() == 0 {
            return "The area is picked clean.".to_string();
        }
        if available.iter().all(|p| REFERENCE_HERBS[*p].tier > self.discoveries.herb_tier) {
            return "You don't recognize any herbs here.".to_string();
        }
        let found = available.remove(0);
        if REFERENCE_HERBS[found].tier > self.discoveries.herb_tier {
            // Return it to the back.
            available.push(found);
            if count <= 1 {
                return "You don't recognize this plant. You leave it be and keep looking.".to_string();
            }
            return format!("You don't recognize this plant. You leave it be and keep looking.\n{}", self.forage(count - 1));
        }
        let result = format!("You collected {}.", found.to_string());
        self.discoveries.mark_herb_found(found, self.current_region);
        self.discoveries.herbs_gathered += 1;
        self.satchel.push(found.to_ingredient());
        if count <= 1 {
            return result;
        }
        format!("{}\n{}", result, self.forage(count - 1))
    }

    fn take_ingredient(&mut self, params: &str, filter: impl Fn(&Ingredient) -> bool) -> Result<Ingredient, String> {
        if params == "" {
            if self.has_cauldron() {
                let c = self.cauldron.take();
                return match c {
                    Some(ingredient) => Ok(ingredient),
                    None => Err("The cauldron is empty".to_string()),
                }
            };
            return Err("Specify an ingredient".to_string())
        }
        if let Some(pos) = self.satchel.iter().position(|x| filter(x) && params.starts_with(x.full_name().as_str())) {
            return Ok(self.satchel.remove(pos));
        }
        if let Some(pos) = self.satchel.iter().position(|x| filter(x) && params.starts_with(x.brew_name().as_str())) {
            return Ok(self.satchel.remove(pos));
        }
        if let Some(pos) = self.satchel.iter().position(|x| filter(x) && params.starts_with(x.base_name().as_str())) {
            return Ok(self.satchel.remove(pos));
        }
        if let Some(pos) = UNLIMITED_INGREDIENTS.iter().position(|x| x.matches_name(params)) {
            return Ok(UNLIMITED_INGREDIENTS[pos].clone());
        }
        if let Some(_pos) = self.infusion_shelf.iter().position(|x| x.full_name() == params) {
            return Err("Wait for that to finish infusing first.".to_string())
        }
        Err(format!("You have no such ingredient: {}", params))
    }

    pub fn bottle_named(&mut self, params: &str) -> String {
        if params == "bottle" {
            return "Nice try. You can't fit a bottle inside a bottle.".to_string();
        }
        // Already checked in bottle(ingredient), but we check here too to avoid taking an item out of the cauldron and putting it back inthe satchel without bottling it
        if self.empty_bottles <= 0 {
            return "You don't have an empty glass bottle. Buy more bottles, or sell or dump out your potions. Customers may or may not return the empty bottle afterwards.".to_string();
        }
        match self.take_ingredient(&params, |ingr: &Ingredient| ingr.container == Container::None) {
            Ok(mut ingr) => {
                let result = self.bottle(&mut ingr);
                self.satchel.push(ingr);
                result.unwrap_or_else(|e| e)
            }
            Err(e) => e,
        }
    }

    fn bottle(&mut self, ingredient: &mut Ingredient) -> Result<String, String> {
        match ingredient.container {
            Container::Bottle => Err(format!("The {} is already bottled.", ingredient.full_name())),
            Container::None => {
                if self.empty_bottles <= 0 {
                    return Err("You don't have an empty glass bottle. Buy more bottles, or sell or dump out your potions. Customers may or may not return the empty bottle afterwards.".to_string());
                }
                self.empty_bottles -= 1;
                let result = format!("You put the {} into a clean bottle.", ingredient.full_name());
                ingredient.container = Container::Bottle;
                Ok(result)
            }
        }
    }

    fn dump_cauldron(&mut self) -> String {
        if !self.has_cauldron() {
            return "There's no cauldron to dump here.".to_string();
        }
        match &self.cauldron {
            Some(work) => {
                let descr = work.show_in_progress(&self.discoveries);
                self.cauldron = None;
                format!("Dumped from cauldron: {}", descr)
            },
            None => "The cauldron is already empty".to_string(),
        }
    }

    pub fn dump(&mut self, params: &str) -> String {
        if params == "" {
            return self.dump_cauldron();
        }
        if let Some(pos) = self.satchel.iter().position(|x| x.container == Container::Bottle && x.matches_name(params)) {
            let mut bottled = self.satchel.remove(pos);
            self.empty_bottles += 1;
            return match bottled.solvent {
                Solvent::Vivo | Solvent:: Air => {
                    bottled.container = Container::None;
                    let result = format!("Unbottled {}.", bottled.full_name());
                    self.satchel.push(bottled);
                    result
                }
                _ => format!("Discarded {} and washed bottle.", bottled.full_name())
            }
        }
        if let Some(pos) = self.infusion_shelf.iter().position(|x| x.container == Container::Bottle && x.full_name() == params) {
            let bottled = self.infusion_shelf.remove(pos);
            self.empty_bottles += 1;
            return format!("Discarded infusing {}.", bottled.full_name())
        }
        return "You can't find anything to dump.".to_string();
    }

    pub fn stir(&mut self) -> String {
        if !self.has_cauldron() {
            return "You see nothing to stir.".to_string();
        }

        let evaporated = match &mut self.cauldron {
            Some(ingredient) => format!("{}\n{}", ingredient.boil(&mut self.discoveries), ingredient.show_in_progress(&self.discoveries)),
            None => return "The cauldron is empty.".to_string(),
        };

        let destabilized = self.tick_elemental_stability();
        match destabilized {
            Some(message) => format!("{}\n{}", evaporated, message),
            None => evaporated,
        }
    }

    fn fill_cauldron(&mut self, ingredient: &Ingredient) -> String {
        if !self.has_cauldron() {
            return "You don't have the equipment to brew potions out here.".to_string();
        }
        match &self.cauldron {
            Some(_) => "Specify an ingredient.".to_string(),
            None => {
                let mut ingredient = ingredient.clone();
                ingredient.container = Container::None;
                let name = ingredient.full_name();
                ingredient.update_effect(&mut self.discoveries);
                let descr = ingredient.show_in_progress(&self.discoveries);
                self.cauldron = Some(ingredient);
                format!("You pour {0} into the cauldron and bring it to a boil.\n{1}", name, descr)
            }
        }
    }

    pub fn decoct_named(&mut self, params: &str) -> String {
        if !self.has_cauldron() {
            return "You don't have the equipment to brew potions out here.".to_string();
        }

        if params == "" {
            self.fill_cauldron(&WATER)
        } else {
            match self.take_ingredient(&params, |_| true) {
                Ok(ingr) => self.decoct(ingr),
                Err(e) => e,
            }
        }
    }

    fn decoct(&mut self, addition: Ingredient) -> String {
        if addition.container == Container::Bottle {
            self.empty_bottles += 1;
        }
        let mut added: Option<String> = None;
        if self.cauldron.is_none() {
            if let Solvent::Water | Solvent::Ether | Solvent::Oil = addition.solvent {
                return self.fill_cauldron(&addition);
            }
            added = Some(self.fill_cauldron(&WATER));
        }
        let decocted = self.cauldron.as_mut().unwrap().decoct(&addition, &mut self.discoveries);
        let destabilized = self.tick_elemental_stability();
        match (added, destabilized) {
            (Some(added), Some(destabilized)) => format!("{}\n{}\n{}", added, decocted, destabilized),
            (None, Some(destabilized)) => format!("{}\n{}", decocted, destabilized),
            (Some(added), None) => format!("{}\n{}", added, decocted),
            (None, None) => decocted,
        }
    }

    pub fn infuse_named(&mut self, params: &str) -> String {
        if !self.discoveries.infusion_known() {
            return "You don't know how to do that yet.".to_string();
        }
        let base = match self.take_ingredient(params, |_| true) {
            Ok(ingredient) => ingredient,
            Err(result) => return result,
        };
        let remainder = match base.search_remainder(params) {
            Some(needle) => needle,
            None => return self.infuse(WATER.clone(), base)
        };
        let addition = match self.take_ingredient(remainder, |_| true) {
            Ok(ingredient) => ingredient,
            Err(result) => return result,
        };
        self.infuse(base, addition)
    }

    fn infuse(&mut self, base: Ingredient, addition: Ingredient) -> String {
        let kind = match base.infusion_kind(&addition) {
            Ok(kind) => kind,
            Err(message) => {
                match addition.infusion_kind(&base) {
                    Ok(_) => return self.infuse(addition, base),
                    Err(_) => {
                        self.satchel.push(base);
                        self.satchel.push(addition);
                        return message;
                    }
                }
            }
        };
        
        let mut base = base;
        match base.container {
            Container::Bottle => match addition.container {
                Container::Bottle => self.empty_bottles += 1,
                Container::None => (),
            },
            Container::None if addition.container == Container::Bottle => base.container = Container::Bottle,
            Container::None => match self.bottle(&mut base) {
                Ok(_result) => (),
                Err(result) => {
                    self.satchel.push(base);
                    self.satchel.push(addition);
                    return result;
                },
            },
        }
        base.kind = kind;
        let result = base.infuse(&addition, &mut self.discoveries);
        self.infusion_shelf.push(base);
        format!("Bottle of [{}] added to shelf to infuse over time.", result)
    }

    fn tick_elemental_stability(&mut self) -> Option<String> {
        let brew = self.cauldron.as_mut()?;
        if brew.is_unstable(Element::Taint) {
            let num_taintable = self.infusion_shelf.len() + self.satchel.len();
            let taint_instability = brew.elements[Element::Taint][Modifier::Provide] - brew.elements[Element::Taint][Modifier::Stabilize];
            for _ in 0..taint_instability {
                if num_taintable <= 0 {
                    break;
                }
                let r = rand::rng().random_range(0..num_taintable);
                if r < self.infusion_shelf.len() {
                    self.infusion_shelf[r].taint(&mut self.discoveries);
                } else {
                    self.satchel[r - self.infusion_shelf.len()].taint(&mut self.discoveries);
                }
            }
            self.cauldron = None;
            return Some(Element::Taint.unstable_message().to_string());
        }
        for (element, status) in brew.elements {
            match element {
                Element::Taint => (), // Already handled
                Element::Water | Element::Light | Element::Thunder | Element::Ice | Element::Air => (), // Handled next
                Element::Fire | Element::Earth | Element::Spirit | Element::Shadow | Element::Mana | Element::Void => {
                    if status[Modifier::Provide] - status[Modifier::Stabilize] > element.base_stability() {
                        self.cauldron = None;
                        return Some(element.unstable_message().to_string());
                    }
                }
            }
        }
        if brew.is_unstable(Element::Ice) {
            // TODO: Consider doing something interesting with the water element here
            self.cauldron = Some(WATER.clone());
            return Some(Element::Ice.unstable_message().to_string());
        }
        let mut messages = Vec::new();
        if brew.is_unstable(Element::Water) {
            let mut all_elements = Vec::new();
            let mut elemental_total = 0;
            for (element, modifiers) in brew.elements {
                elemental_total += modifiers[Modifier::Provide];
                all_elements.push((element, modifiers[Modifier::Provide]));
            }
            let mut r = rand::rng().random_range(0..elemental_total);
            for (element, amount) in all_elements {
                if r > amount {
                    r -= amount;
                }
                else {
                    brew.elements[element][Modifier::Provide] -= 1;
                    messages.push(format!("{} {}.", Element::Water.unstable_message(), element));
                    break;
                }
            }
        }
        if brew.is_unstable(Element::Light) {
            brew.elements[Element::Light][Modifier::Provide] = 0;
            brew.elements[Element::Light][Modifier::Stabilize] = 0;
            brew.elements[Element::Light][Modifier::Strengthen] = 0;
            messages.push(Element::Light.unstable_message().to_string());
        }
        if brew.is_unstable(Element::Thunder) {
            for (element, status) in brew.elements {
                for (modifier, _amount) in status {
                    brew.elements[element][modifier] = status[modifier] * 3 / 4;
                }
            }
            messages.push(Element::Thunder.unstable_message().to_string());
        }
        if brew.is_unstable(Element::Air) {
            for (element, status) in brew.elements {
                for (modifier, _amount) in status {
                    brew.elements[element][modifier] /= 2;
                }
            }
            messages.push(Element::Air.unstable_message().to_string());
        }
        for (element, status) in brew.elements {
            if status[Modifier::Provide] - status[Modifier::Stabilize] == element.base_stability() {
                messages.push(element.warning().to_string());
            }
        }
        brew.update_effect(&mut self.discoveries);

        if messages.is_empty() {
            None
        } else {
            Some(format!("{}\n{}", messages.join("\n"), brew.show_in_progress(&self.discoveries)))
        }
    }

    pub fn buy(&mut self, params: &str) -> String {
        if self.current_region != RegionEnum::Village {
            return "There's no one here to buy from.".to_string()
        }
        let bottle_price = 2;
        let spirits_price = 8;
        let oil_price = 24;
        match params {
            "bottle"|"b"|"" => {
                if self.money < bottle_price {
                    return format!("You only have {} silver and can't afford {} for a bottle", self.money, bottle_price);
                }
                self.money -= bottle_price;
                self.empty_bottles += 1;
                "You bought an empty bottle.".to_string()
            }
            "spirits" => {
                if !self.discoveries.spirits_unlocked() {
                    return "You're not skilled enough to work with that yet.".to_string();
                }
                if self.money < spirits_price {
                    return format!("You only have {} silver and can't afford {} for some spirits", self.money, spirits_price);
                }
                self.money -= spirits_price;
                self.satchel.push(ETHER.clone());
                "You bought spirits.".to_string()
            }
            "oil" => {
                if !self.discoveries.oil_unlocked() {
                    return "You're not skilled enough to work with that yet.".to_string();
                }
                if self.money < oil_price {
                    return format!("You only have {} silver and can't afford {} for oil", self.money, oil_price);
                }
                self.money -= oil_price;
                self.satchel.push(OIL.clone());
                "You bought oil.".to_string()
            }
            _ => format!("The village doesn't sell '{}'", params)
        }
    }

    pub fn sell(&mut self, params: &str) -> String {
        if self.current_region != RegionEnum::Village {
            return "There's no one here to sell to.".to_string()
        }
        if params == "soul" {
            return "We deal in spirits, but not souls.".to_string();
        }
        if params == "bottle" {
            if self.empty_bottles <= 0 {
                return "You have no bottles to sell".to_string();
            }
            let value = Container::Bottle.sale_value();
            self.empty_bottles -= 1;
            self.money += value;
            return format!("Sold a bottle for {} silver", value);
        }
        let item = match self.take_ingredient(params, |_| true) {
            Ok(ingredient) => ingredient,
            Err(result) => return result,
        };
        let value = item.sale_value();
        self.money += value;
        match item.container {
            Container::Bottle => { self.bottles_sold += 1; }
            Container::None => (),
        }
        format!("Sold {} for {} silver pieces", item.full_name(), value)
    }

    pub fn advance_time(&mut self) -> String {
        let regional_message = match self.regions[self.current_region].sleep_result {
            Ok(message) => message,
            Err(message) => return message.to_string(),
        };

        // Ingredients dry out or rot
        let herb_changes = self.satchel.iter_mut().filter_map(|i| i.advance_time()).collect::<Vec<String>>().join("\n");
        // Infusions complete
        let infused = self.infusion_shelf.len();
        self.satchel.append(&mut self.infusion_shelf);
        // Customers return bottles
        let prev_bottles = self.empty_bottles;
        for _i in 0..self.bottles_sold {
            if rand::random_bool(0.5) {
                self.empty_bottles += 1;
                self.bottles_sold -= 1;
            } else if rand::random_bool(0.1) {
                self.bottles_sold -= 1;
            }
        }
        let bottles_returned = self.empty_bottles - prev_bottles;
        // Herbs regrow
        for (e, region) in self.regions.iter_mut() {
            region.regrow(&e);
        }
        let xp = self.discoveries.update();
        let mut response = Vec::new();
        if !herb_changes.is_empty() {
            response.push(herb_changes);
        }
        if infused > 0 {
            response.push(format!("Completed {} infusions.", infused));
        }
        if bottles_returned > 0 {
            response.push(format!("Customers returned {} empty bottles.", bottles_returned));
        }

        response.push(regional_message.to_string());

        if let Some(x) = xp {
            response.push(x);
        }

        response.join("\n")
    }

    pub fn experience(&self) -> String {
        self.discoveries.status()
    }

    pub fn look(&mut self) -> String {
        let region = &self.regions[self.current_region];
        let mut vec = Vec::new();
        vec.push(region.name.to_string());
        vec.push(region.description.to_string());
        if let Some(status) = region.status(&self.discoveries) {
            vec.push(status);
        }
        if self.current_region == RegionEnum::Hut {
            match &self.cauldron {
                Some(brew) => vec.push(format!("Cauldron: {}\n    {}", brew.full_name(), brew.show_in_progress(&self.discoveries))),
                None => vec.push("Cauldron: empty".to_string()),
            }
        }

        vec.join("\n")
    }
}
