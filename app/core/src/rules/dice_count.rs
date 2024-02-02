use serde::{Deserialize, Serialize};

use super::DiceType;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct DiceCount(u8, DiceType);
