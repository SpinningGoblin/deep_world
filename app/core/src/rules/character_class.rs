use serde::{Deserialize, Serialize};

use super::DiceCount;

/// The class that you will be able to level as your character levels.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterClass {
    /// Unique way to identify this character class vs another. Must be unique across ruleset.
    pub id: String,
    /// Name for the class that would get displayed to a user or player.
    pub name: String,
    /// The dice that the player character would roll if taking a level of this class
    /// when the appropriate amount of XP or milestones are reached.
    pub health_dice: DiceCount,
}
