use std::{path::Path, io::BufReader, fs::File};

use config::{Config};


pub fn get_config() -> Config {
    Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Could not load config from env properties") // ok to panic, if the config cannot be loaded
}

pub fn get_translations() -> Result<serde_json::Value,serde_json::Error> {
    let filename = "translations.json";

    let path = Path::new(filename);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    
    Ok(serde_json::from_reader(reader).unwrap())
}