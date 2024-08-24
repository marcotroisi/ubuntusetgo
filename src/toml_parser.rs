use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    os: Os,
    packages: HashMap<String, Vec<String>>,
    configs: Configs,
}

#[derive(Deserialize)]
struct Os {}

// #[derive(Deserialize)]
// struct Packages {}

#[derive(Deserialize)]
struct Configs {}

pub fn parse(path: &str) -> Result<Config, anyhow::Error> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

#[test]
fn parse_toml() {
    let result: Config = parse("testconfig/ubuntusetgo.toml").unwrap();
    let expected = HashMap::from([(
        "apt".to_string(),
        vec!["neovim".to_string(), "alacritty".to_string()],
    )]);

    assert_eq!(result.packages, expected);
}
