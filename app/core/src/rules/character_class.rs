use serde::{Deserialize, Serialize};

use super::DiceCount;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterClass {
    pub id: String,
    pub name: String,
    pub health_dice: DiceCount,
}
