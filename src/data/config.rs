use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

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
        hostname: match env::var("SERVICE_HOST") {
            Ok(host) => host,
            Err(_) => "127.0.0.1".to_string(),
        },
        port: match env::var("SERVICE_PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 7674,
        }
    };
}
