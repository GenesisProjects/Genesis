use config::*;
use std::collections::HashMap;
use std::path::Path;

pub fn version() -> String {
    let mut config = Config::default();
    let path = Path::new("../config/application.json");
    config.merge(File::from(path))
        .expect("Could not open config");
    config.get_str("version").expect("Could not find version")
}

pub fn load_table(service: &str) -> HashMap<String, Value> {
    let mut config = Config::default();
    let path = Path::new("../config/application.json");
    config.merge(File::from(path))
        .expect("Could not open config");
    config.get_table(service).expect("Could not load table")
}
