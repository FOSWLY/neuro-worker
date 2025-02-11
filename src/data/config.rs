use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CargoConfig {
    package: CargoPackage,
}

#[derive(Deserialize)]
pub struct CargoPackage {
    version: String,
}

pub struct Config {
    pub version: String,
    pub hostname: String,
    pub port: u16,
}

lazy_static! {
    static ref CARGO_CONFIG: CargoConfig =
        toml::from_str(include_str!("../../Cargo.toml")).unwrap();
    pub static ref CONFIG: Config = Config {
        version: CARGO_CONFIG.package.version.clone(),
        hostname: "127.0.0.1".to_string(),
        port: 7674
    };
}
