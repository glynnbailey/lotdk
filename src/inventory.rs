use crate::assets::{ArmorSlot, ItemKind, ItemType};

#[derive(Clone)]
pub struct InventoryItem {
    pub item_id: String,
    pub quantity: i32,
}

pub struct Inventory {
    items: Vec<InventoryItem>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item_id: String, quantity: i32) {
        if let Some(existing) = self.items.iter_mut().find(|item| item.item_id == item_id) {
            existing.quantity += quantity;
        } else {
            self.items.push(InventoryItem { item_id, quantity });
        }
    }

    pub fn remove_item(&mut self, item_id: &str, quantity: i32) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item.item_id == item_id) {
            let item = &mut self.items[pos];
            if item.quantity >= quantity {
                item.quantity -= quantity;
                if item.quantity == 0 {
                    self.items.remove(pos);
                }
                return true;
            }
        }
        false
    }

    pub fn has_item(&self, item_id: &str) -> bool {
        self.items.iter().any(|item| item.item_id == item_id)
    }

    pub fn get_quantity(&self, item_id: &str) -> i32 {
        self.items.iter().find(|item| item.item_id == item_id).map(|item| item.quantity).unwrap_or(0)
    }

    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }
}

pub struct Equipment {
    pub weapon: Option<String>,
    pub offhand: Option<String>,
    pub head: Option<String>,
    pub chest: Option<String>,
    pub legs: Option<String>,
    pub feet: Option<String>,
    pub hands: Option<String>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            weapon: None,
            head: None,
            chest: None,
            legs: None,
            feet: None,
            hands: None,
            offhand: None,
        }
    }

    pub fn can_equip(&self, item: &ItemKind) -> bool {
        match &item.item_type {
            ItemType::Weapon { .. } => true,
            ItemType::Armor { .. } => true,
            _ => false,
        }
    }

    pub fn equip_item(&mut self, item: &ItemKind) -> Result<Option<String>, String> {
        match &item.item_type {
            ItemType::Weapon { .. } => {
                let old = self.weapon.take();
                self.weapon = Some(item.id.clone());
                Ok(old)
            }
            ItemType::Armor { slot, .. } => {
                let old = match slot {
                    ArmorSlot::Head => self.head.replace(item.id.clone()),
                    ArmorSlot::Chest => self.chest.replace(item.id.clone()),
                    ArmorSlot::Legs => self.legs.replace(item.id.clone()),
                    ArmorSlot::Feet => self.feet.replace(item.id.clone()),
                    ArmorSlot::Hands => self.hands.replace(item.id.clone()),
                    ArmorSlot::Offhand => self.offhand.replace(item.id.clone()),
                };
                Ok(old)
            }
            _ => Err("Cannot equip this item type".to_string()),
        }
    }

    pub fn unequip_slot(&mut self, slot: &ArmorSlot) -> Option<String> {
        match slot {
            ArmorSlot::Head => self.head.take(),
            ArmorSlot::Chest => self.chest.take(),
            ArmorSlot::Legs => self.legs.take(),
            ArmorSlot::Feet => self.feet.take(),
            ArmorSlot::Hands => self.hands.take(),
            ArmorSlot::Offhand => self.offhand.take(),
        }
    }

    pub fn unequip_weapon(&mut self) -> Option<String> {
        self.weapon.take()
    }

    // For UI display, you can still iterate in order:
    pub fn iter_slots(&self) -> Vec<(&str, &Option<String>)> {
        vec![("Weapon", &self.weapon), ("Offhand", &self.offhand), ("Head", &self.head), ("Chest", &self.chest), ("Legs", &self.legs), ("Feet", &self.feet), ("Hands", &self.hands)]
    }
}
