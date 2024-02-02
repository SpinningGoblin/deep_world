use classes::DefaultClasses;
use deep_world_core::rules::CharacterClass;

mod classes;

pub fn default_ruleset() -> Vec<CharacterClass> {
    DefaultClasses::all()
}
