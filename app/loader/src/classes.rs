use deep_world_core::rules::CharacterClass;

pub struct DefaultClasses;

impl DefaultClasses {
    pub fn weapon_master() -> &'static str {
        include_str!("../../data/classes/weapon_master.toml")
    }

    pub fn all() -> Vec<CharacterClass> {
        let weapon_master: CharacterClass = toml::from_str(Self::weapon_master()).unwrap();

        vec![weapon_master]
    }
}
