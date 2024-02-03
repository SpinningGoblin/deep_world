use deep_world_core::rules::Ancestry;

pub struct DefaultAncestries;

impl DefaultAncestries {
    pub fn goblin() -> &'static str {
        include_str!("../../data/ancestries/goblin.yaml")
    }

    pub fn all() -> Vec<Ancestry> {
        vec![serde_yaml::from_str(Self::goblin()).unwrap()]
    }
}
