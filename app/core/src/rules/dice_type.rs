use serde::{Deserialize, Serialize};

/// A certain type of dice, with a specific number of sides.
/// Will be used to describe how to roll attacks, health, damage, etc.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[allow(dead_code)]
impl DiceType {
    pub fn num_sides(&self) -> u8 {
        match self {
            DiceType::D4 => 4,
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D12 => 12,
            DiceType::D20 => 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::dice_type::DiceType;

    #[test]
    pub fn num_sides() {
        assert_eq!(DiceType::D4.num_sides(), 4);
        assert_eq!(DiceType::D6.num_sides(), 6);
        assert_eq!(DiceType::D8.num_sides(), 8);
        assert_eq!(DiceType::D10.num_sides(), 10);
        assert_eq!(DiceType::D12.num_sides(), 12);
        assert_eq!(DiceType::D20.num_sides(), 20);
    }
}
