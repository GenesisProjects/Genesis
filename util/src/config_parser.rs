use config;
use serde;
use std::collections::HashMap;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
	pub static ref SETTINGS: RwLock<config::Config> = {
	    let mut setting = config::Config::default();
	    setting.merge(config::File::with_name("application")).unwrap();
        RwLock::new(setting)
	};
}

