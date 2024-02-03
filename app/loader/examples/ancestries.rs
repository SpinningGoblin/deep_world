use loader::default_ruleset;

pub fn main() {
    let ruleset = default_ruleset();
    println!("{}", serde_json::to_string(&ruleset.ancestries).unwrap());
}
