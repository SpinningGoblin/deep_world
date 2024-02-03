use serde::{Deserialize, Serialize};

use super::DiceCount;

/// The class that you will be able to level as your character levels.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterClass {
    pub id: String,
    pub name: String,
    pub health_dice: DiceCount,
}
