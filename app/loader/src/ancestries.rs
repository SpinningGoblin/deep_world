use deep_world_core::rules::Ancestry;

pub struct DefaultAncestries;

impl DefaultAncestries {
    pub fn goblin() -> &'static str {
        include_str!("../../data/ancestries/goblin.toml")
    }

    pub fn crystalith() -> &'static str {
        include_str!("../../data/ancestries/crystalith.toml")
    }

    pub fn all() -> Vec<Ancestry> {
        vec![
            toml::from_str(Self::goblin()),
            toml::from_str(Self::crystalith()),
        ]
        .into_iter()
        .filter_map(|it| it.ok())
        .collect()
    }
}
