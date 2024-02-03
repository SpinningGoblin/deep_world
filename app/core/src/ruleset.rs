use serde::{Deserialize, Serialize};

use crate::rules::{Ancestry, CharacterClass};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuleSet {
    pub character_classes: Vec<CharacterClass>,
    pub ancestries: Vec<Ancestry>,
}
