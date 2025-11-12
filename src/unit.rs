/// Represents the type of a combat unit.
/// Each type defines its own base stats.
pub enum UnitType {
    Large,
    Medium,
    Light,
}

impl UnitType {
    /// Returns (health, base_damage) stats for each unit type.
    pub fn stats(&self) -> (i32, i32) {
        match self {
            UnitType::Large => (120, 8),
            UnitType::Medium => (90, 12),
            UnitType::Light => (60, 18),
        }
    }

    /// Returns a human-readable string for printing.
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitType::Large => "Large",
            UnitType::Medium => "Medium",
            UnitType::Light => "Light",
        }
    }
}

/// Represents a single unit in the game.
/// Each unit owns its name, health, and damage values.
pub struct Unit {
    pub name: String,
    pub unit_type: UnitType,
    pub health: i32,
    pub base_damage: i32,
}

impl Unit {
    /// Creates a new `Unit` instance given a name and type.
    pub fn new(name: &str, unit_type: UnitType) -> Self {
        let (hp, dmg) = unit_type.stats();
        Self {
            name: name.to_string(),
            unit_type,
            health: hp,
            base_damage: dmg,
        }
    }

    /// Returns true if the unit still has health remaining.
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
