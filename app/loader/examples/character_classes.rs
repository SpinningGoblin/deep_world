use loader::default_ruleset;

pub fn main() {
    let classes = default_ruleset();
    println!("{}", serde_json::to_string(&classes).unwrap());
}
