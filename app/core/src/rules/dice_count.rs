use serde::{Deserialize, Serialize};

use super::DiceType;

/// A count of dice to roll for any reason. e.g. 2d6
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct DiceCount(u8, DiceType);
