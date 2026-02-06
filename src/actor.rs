use crate::{
    actor_manager::ActorManager,
    assets::{ASSETS, ArmorSlot, ConsumableEffect, ItemKind, ItemType},
    inventory::{Equipment, Inventory},
    map_manager::MapManager,
    pathfinding::a_star,
    playing::Action,
    position::Position,
};
use crossterm::style::Color;
use rand::seq::SliceRandom;

pub struct Actor {
    kind_id: String,
    character_stats: Option<CharacterStats>,

    position: Position,
    health: i32,
    ai_state: ActorAiState,
}

#[derive(Debug, Clone)]
pub enum ActorAiState {
    Idle,
    TargetingActor(usize),
    InvestigatingPosition(Position),
}

impl Actor {
    fn character_stats_mut(&mut self) -> Result<&mut CharacterStats, String> {
        self.character_stats.as_mut().ok_or_else(|| "Actor has no character stats".to_string())
    }

    pub fn new(kind_id: String, character_stats: Option<CharacterStats>, position: Position) -> Self {
        let kind = ASSETS.actor_kinds.iter().find(|k| k.id == kind_id).unwrap();

        Self {
            kind_id,
            character_stats,

            position,
            health: kind.health,
            ai_state: ActorAiState::Idle,
        }
    }

    pub fn glyph(&self) -> (char, Color) {
        let kind = ASSETS.actor_kinds.iter().find(|k| k.id == self.kind_id).unwrap();
        (kind.glyph, kind.color)
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn max_health(&self) -> i32 {
        let kind = ASSETS.actor_kinds.iter().find(|k| k.id == self.kind_id).unwrap();
        kind.health
    }

    pub fn speed(&self) -> u32 {
        let kind = ASSETS.actor_kinds.iter().find(|k| k.id == self.kind_id).unwrap();
        kind.speed
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_state(&mut self, state: ActorAiState) {
        self.ai_state = state;
    }

    pub fn melee_attack_roll(&self) -> i32 {
        self.weapon_damage()
    }

    pub fn apply_damage(&mut self, damage: i32) -> ApplyDamageResult {
        let actual_damage = (damage - self.total_defense()).max(1); // Minimum 1 damage
        self.health -= actual_damage;
        if self.health <= 0 { ApplyDamageResult::ActorDied } else { ApplyDamageResult::None }
    }

    // Inventory methods
    pub fn inventory(&self) -> Option<&Inventory> {
        match self.character_stats.as_ref() {
            Some(stats) => Some(&stats.inventory),
            None => None,
        }
    }

    pub fn inventory_mut(&mut self) -> Option<&mut Inventory> {
        match self.character_stats.as_mut() {
            Some(stats) => Some(&mut stats.inventory),
            None => None,
        }
    }

    pub fn add_item(&mut self, item_id: String, quantity: i32) -> Result<(), String> {
        let inventory = self.inventory_mut().ok_or_else(|| "Actor has no inventory".to_string())?;
        inventory.add_item(item_id, quantity);
        Ok(())
    }

    pub fn remove_item(&mut self, item_id: &str, quantity: i32) -> Result<bool, String> {
        let inventory = self.inventory_mut().ok_or_else(|| "Actor has no inventory".to_string())?;
        Ok(inventory.remove_item(item_id, quantity))
    }

    // Equipment methods
    pub fn equipment(&self) -> Option<&Equipment> {
        match self.character_stats.as_ref() {
            Some(stats) => Some(&stats.equipment),
            None => None,
        }
    }

    pub fn equipment_mut(&mut self) -> Option<&mut Equipment> {
        match self.character_stats.as_mut() {
            Some(stats) => Some(&mut stats.equipment),
            None => None,
        }
    }

    pub fn equip_item_from_inventory(&mut self, item_id: &str) -> Result<(), String> {
        let stats = self.character_stats_mut()?;

        // Check if item is in inventory and remove it
        if !stats.inventory.remove_item(item_id, 1) {
            return Err("Item not in inventory".to_string());
        }

        // Get item kind from game data
        let item_kind = ASSETS.item_kinds.iter().find(|kind| kind.id == item_id).ok_or("Unknown item type")?;

        // Try to equip the item
        match stats.equipment.equip_item(item_kind) {
            Ok(old_item) => {
                // Add old item back to inventory if there was one
                if let Some(old_item_id) = old_item {
                    stats.inventory.add_item(old_item_id, 1);
                }
                Ok(())
            }
            Err(err) => {
                // Equipping failed, put the item back in inventory
                stats.inventory.add_item(item_id.to_string(), 1);
                Err(err)
            }
        }
    }

    pub fn equip_item(&mut self, item_kind: &ItemKind) -> Result<Option<String>, String> {
        let stats = self.character_stats_mut()?;
        stats.equipment.equip_item(item_kind)
    }

    // Consumable item usage
    pub fn use_consumable(&mut self, item_id: &str) -> Result<String, String> {
        let stats = self.character_stats_mut()?;

        // Check if item is in inventory
        if !stats.inventory.has_item(item_id) {
            return Err("Item not in inventory".to_string());
        }

        // Get item kind from game data
        let item_kind = ASSETS.item_kinds.iter().find(|kind| kind.id == item_id).ok_or("Unknown item type")?;

        // Check if item is consumable
        let effect = match &item_kind.item_type {
            ItemType::Consumable { effect } => effect,
            _ => return Err("Item is not consumable".to_string()),
        };

        // Apply the effect
        let result_message = self.apply_consumable_effect(effect)?;

        Ok(result_message)
    }

    fn apply_consumable_effect(&mut self, effect: &ConsumableEffect) -> Result<String, String> {
        match effect {
            ConsumableEffect::Heal { amount } => {
                let old_health = self.health;
                self.health = (self.health + amount).min(self.max_health());
                let actual_healing = self.health - old_health;
                Ok(format!("Restored {} health", actual_healing))
            }
            ConsumableEffect::RestoreMana { amount } => {
                // TODO: Implement mana system
                Ok(format!("Would restore {} mana (not implemented)", amount))
            }
            ConsumableEffect::Buff { stat, amount, duration } => {
                // TODO: Implement buff system
                Ok(format!("Would apply +{} {} for {} turns (not implemented)", amount, stat, duration))
            }
        }
    }

    pub fn unequip_armor_slot(&mut self, slot: &ArmorSlot) -> Option<String> {
        let stats = self.character_stats_mut().ok()?;
        if let Some(item_id) = stats.equipment.unequip_slot(slot) {
            stats.inventory.add_item(item_id.clone(), 1);
            return Some(item_id);
        }
        None
    }

    pub fn unequip_weapon(&mut self) -> Option<String> {
        let stats = self.character_stats_mut().ok()?;
        if let Some(item_id) = stats.equipment.unequip_weapon() {
            stats.inventory.add_item(item_id.clone(), 1);
            return Some(item_id);
        }
        None
    }

    // Calculate total defense from equipped armor
    pub fn total_defense(&self) -> i32 {
        let mut defense = 0;

        if let Some(stats) = self.character_stats.as_ref() {
            for (_, equipped_item) in stats.equipment.iter_slots() {
                if let Some(item_id) = equipped_item {
                    if let Some(item_kind) = ASSETS.item_kinds.iter().find(|k| &k.id == item_id) {
                        if let ItemType::Armor { defense: item_defense, .. } = &item_kind.item_type {
                            defense += item_defense;
                        }
                    }
                }
            }
        }

        defense
    }

    pub fn weapon_damage(&self) -> i32 {
        if let Some(stats) = self.character_stats.as_ref() {
            if let Some(weapon_id) = &stats.equipment.weapon {
                if let Some(weapon_kind) = ASSETS.item_kinds.iter().find(|k| &k.id == weapon_id) {
                    if let ItemType::Weapon { damage, .. } = &weapon_kind.item_type {
                        return *damage;
                    }
                }
            }
        }
        // Base unarmed damage
        1
    }

    pub fn ai_turn(&self, actor_id: usize, actors: &ActorManager, map: &MapManager) -> (ActorAiState, Action) {
        let mut current_state = self.ai_state.clone();
        let visible_tiles = map.shadowcast(self.position);

        loop {
            match &current_state {
                ActorAiState::Idle => {
                    // try to find a target
                    let mut possible_targets = Vec::new();
                    for visible_position in &visible_tiles {
                        if let Some(tile) = map.get_tile(*visible_position) {
                            if let Some(other_actor_id) = tile.actor_id() {
                                if other_actor_id != actor_id {
                                    let target_actor = actors.get_actor(other_actor_id).unwrap();
                                    if !self.is_friendly_towards(target_actor) {
                                        possible_targets.push(other_actor_id);
                                    }
                                }
                            }
                        }
                    }

                    if !possible_targets.is_empty() {
                        possible_targets.shuffle(&mut rand::rng());
                        let target_actor_id = possible_targets.pop().unwrap();
                        current_state = ActorAiState::TargetingActor(target_actor_id);
                        continue;
                    }

                    // no target found, remain idle
                    return (ActorAiState::Idle, Action::Wait);
                }

                ActorAiState::TargetingActor(other_actor_id) => {
                    let target_actor = match actors.get_actor(*other_actor_id) {
                        Some(actor) => actor,
                        None => {
                            // target no longer exists, probably died, go idle
                            current_state = ActorAiState::Idle;
                            continue;
                        }
                    };

                    // make sure target is still visible
                    if !visible_tiles.contains(&target_actor.position()) {
                        // lost sight of target, investigate last known position
                        let target_position = target_actor.position().clone();
                        current_state = ActorAiState::InvestigatingPosition(target_position);
                        continue;
                    }

                    if self.position.is_adjacent(target_actor.position) {
                        // attack target
                        return (ActorAiState::TargetingActor(*other_actor_id), Action::MeleeAttack(*other_actor_id));
                    } else {
                        // move towards target
                        let path = a_star(actors, map, self.position, target_actor.position(), actor_id);
                        if let Some(path) = path {
                            if !path.is_empty() {
                                let next_position = path[0].clone();
                                return (ActorAiState::TargetingActor(*other_actor_id), Action::MoveTo(next_position));
                            }
                        }

                        // cannot path to target, go idle
                        return (ActorAiState::Idle, Action::Wait);
                    }
                }

                ActorAiState::InvestigatingPosition(target_position) => {
                    // check if any targets can be seen from this position
                    let mut possible_targets = Vec::new();
                    for visible_position in &visible_tiles {
                        if let Some(tile) = map.get_tile(*visible_position) {
                            if let Some(other_actor_id) = tile.actor_id() {
                                if other_actor_id != actor_id {
                                    let target_actor = actors.get_actor(other_actor_id).unwrap();
                                    if !self.is_friendly_towards(target_actor) {
                                        possible_targets.push(other_actor_id);
                                    }
                                }
                            }
                        }
                    }
                    if !possible_targets.is_empty() {
                        possible_targets.shuffle(&mut rand::rng());
                        let target_actor_id = possible_targets.pop().unwrap();
                        current_state = ActorAiState::TargetingActor(target_actor_id);
                        continue;
                    }

                    // check if reached investigation position
                    if &self.position == target_position {
                        return (ActorAiState::Idle, Action::Wait);
                    }

                    // move towards investigation position
                    let path = a_star(actors, map, self.position, *target_position, actor_id);
                    if let Some(path) = path {
                        if !path.is_empty() {
                            let next_position = path[0].clone();
                            return (ActorAiState::InvestigatingPosition(target_position.clone()), Action::MoveTo(next_position));
                        }
                    }

                    // cannot reach position, go idle
                    return (ActorAiState::Idle, Action::Wait);
                }
            }
        }
    }

    pub fn is_friendly_towards(&self, other: &Actor) -> bool {
        let self_faction = match self.character_stats.as_ref() {
            Some(stats) => stats.faction.clone(),
            None => match ASSETS.actor_kinds.iter().find(|kind| kind.id == self.kind_id) {
                Some(kind) => kind.faction.clone(),
                None => String::new(),
            },
        };

        let other_faction = match other.character_stats.as_ref() {
            Some(stats) => stats.faction.clone(),
            None => match ASSETS.actor_kinds.iter().find(|kind| kind.id == other.kind_id) {
                Some(kind) => kind.faction.clone(),
                None => String::new(),
            },
        };

        self_faction == other_faction
    }
}

pub struct CharacterStats {
    #[allow(dead_code)]
    name: String,
    inventory: Inventory,
    equipment: Equipment,
    faction: String,
}

impl CharacterStats {
    pub fn new(name: String, faction: String) -> Self {
        Self {
            name,
            inventory: Inventory::new(),
            equipment: Equipment::new(),
            faction,
        }
    }
}

pub enum ApplyDamageResult {
    None,
    ActorDied,
}
