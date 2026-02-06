use core::panic;

use crossterm::style::Color;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub static ASSETS: Lazy<Assets> = Lazy::new(|| load_data("assets"));

pub struct Assets {
    pub actor_kinds: Vec<ActorKind>,
    pub item_kinds: Vec<ItemKind>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
enum DataType {
    #[serde(rename = "actor")]
    Actor(ActorKind),
    #[serde(rename = "item")]
    Item(ItemKind),
}

fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let color_str = match color {
        Color::Reset => panic!("Reset color not supported in serialization"),
        Color::Black => "black",
        Color::DarkGrey => "dark_grey",
        Color::Red => "red",
        Color::DarkRed => "dark_red",
        Color::Green => "green",
        Color::DarkGreen => "dark_green",
        Color::Yellow => "yellow",
        Color::DarkYellow => "dark_yellow",
        Color::Blue => "blue",
        Color::DarkBlue => "dark_blue",
        Color::Magenta => "magenta",
        Color::DarkMagenta => "dark_magenta",
        Color::Cyan => "cyan",
        Color::DarkCyan => "dark_cyan",
        Color::White => "white",
        Color::Grey => "grey",
        Color::Rgb { .. } => panic!("RGB colors not supported in serialization"),
        Color::AnsiValue(_) => panic!("AnsiValue colors not supported in serialization"),
    };
    serializer.serialize_str(color_str)
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let color_str = String::deserialize(deserializer)?;
    match color_str.as_str() {
        "black" => Ok(Color::Black),
        "dark_grey" => Ok(Color::DarkGrey),
        "red" => Ok(Color::Red),
        "dark_red" => Ok(Color::DarkRed),
        "green" => Ok(Color::Green),
        "dark_green" => Ok(Color::DarkGreen),
        "yellow" => Ok(Color::Yellow),
        "dark_yellow" => Ok(Color::DarkYellow),
        "blue" => Ok(Color::Blue),
        "dark_blue" => Ok(Color::DarkBlue),
        "magenta" => Ok(Color::Magenta),
        "dark_magenta" => Ok(Color::DarkMagenta),
        "cyan" => Ok(Color::Cyan),
        "dark_cyan" => Ok(Color::DarkCyan),
        "white" => Ok(Color::White),
        "grey" => Ok(Color::Grey),
        _ => Err(serde::de::Error::custom(format!("Unsupported color: {}", color_str))),
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorKind {
    pub id: String,
    pub name: String,
    pub glyph: char,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub color: Color,
    pub faction: String,

    pub health: i32,
    pub speed: u32,

    #[serde(default = "default_true")]
    pub spawnable: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemKind {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(flatten)]
    pub item_type: ItemType,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "item_type")]
pub enum ItemType {
    #[serde(rename = "weapon")]
    Weapon { damage: i32, weapon_type: WeaponType },
    #[serde(rename = "armor")]
    Armor { defense: i32, slot: ArmorSlot },
    #[serde(rename = "consumable")]
    Consumable { effect: ConsumableEffect },
    #[serde(rename = "misc")]
    Miscellaneous { stackable: bool },
}

#[derive(Clone, Serialize, Deserialize)]
pub enum WeaponType {
    #[serde(rename = "sword")]
    Sword,
    #[serde(rename = "bow")]
    Bow,
    #[serde(rename = "staff")]
    Staff,
}

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ArmorSlot {
    #[serde(rename = "offhand")]
    Offhand,
    #[serde(rename = "head")]
    Head,
    #[serde(rename = "chest")]
    Chest,
    #[serde(rename = "legs")]
    Legs,
    #[serde(rename = "feet")]
    Feet,
    #[serde(rename = "hands")]
    Hands,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ConsumableEffect {
    #[serde(rename = "heal")]
    Heal { amount: i32 },
    #[serde(rename = "mana")]
    RestoreMana { amount: i32 },
    #[serde(rename = "buff")]
    Buff { stat: String, amount: i32, duration: i32 },
}

pub fn load_data(path: &str) -> Assets {
    let mut actor_kinds = Vec::new();
    let mut item_kinds = Vec::new();

    for file in std::fs::read_dir(path).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = std::fs::read_to_string(&path).unwrap();
            let data_types: Vec<DataType> = serde_yaml::from_str(&content).expect("Failed to parse data file");
            for data_type in data_types {
                match data_type {
                    DataType::Actor(actor) => actor_kinds.push(actor),
                    DataType::Item(item) => item_kinds.push(item),
                }
            }
        }
    }

    Assets { actor_kinds, item_kinds }
}
