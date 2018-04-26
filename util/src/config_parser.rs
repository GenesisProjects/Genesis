use config;
use serde;
use std::collections::HashMap;
use std::error::Error;
use std::sync::RwLock;
use std::path::Path;
use std::env;

lazy_static! {
	pub static ref SETTINGS: RwLock<config::Config> = RwLock::new({
	    let mut setting = config::Config::default();
	    let mut path_buff = env::current_dir().unwrap();
	    path_buff.push("config");
	    path_buff.push("application");
	    path_buff.set_extension("json");
	    setting.merge(config::File::from(path_buff)).unwrap();
        setting
	});
}

