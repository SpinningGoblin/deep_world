use loader::default_ruleset;

pub fn main() {
    let ruleset = default_ruleset();
    println!("{}", toml::to_string(&ruleset).unwrap());
}
