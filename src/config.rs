use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;

const config_path: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct DotFile {
    origin_path: String,
    copy_path: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    dot_files: Vec<DotFile>,
}
pub fn load_config() -> Result<Config, &'static str> {
    let config_text: String = fs::read_to_string(&config_path)?.parse()?;
    let config: Result<Config, toml::de::Error> = toml::from_str(&config_text);
    match config {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("Failed to load config: {}", e),
    }
    Ok(());
}
pub fn write_config(config: Config) -> Result<&'static str, &'static str>{
    let mut file = File::create(&config_path)?;
    let toml = toml::to_string(&config).unwrap();
    write!(file, "{}", toml)?;
    file.flush()?;
}