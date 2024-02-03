use ancestries::DefaultAncestries;
use classes::DefaultClasses;
use deep_world_core::RuleSet;

mod ancestries;
mod classes;

pub fn default_ruleset() -> RuleSet {
    RuleSet {
        character_classes: DefaultClasses::all(),
        ancestries: DefaultAncestries::all(),
    }
}
