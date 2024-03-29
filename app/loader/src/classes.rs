use deep_world_core::rules::CharacterClass;

pub struct DefaultClasses;

impl DefaultClasses {
    pub fn fighter() -> &'static str {
        include_str!("../../data/classes/fighter.toml")
    }

    pub fn all() -> Vec<CharacterClass> {
        let fighter: CharacterClass = toml::from_str(Self::fighter()).unwrap();

        vec![fighter]
    }
}
