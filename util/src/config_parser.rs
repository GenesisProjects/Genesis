use config;
use std::sync::RwLock;
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

	pub static ref BLOCK_SETTINGS: RwLock<config::Config> = RwLock::new({
	    let mut setting = config::Config::default();
	    let mut path_buff = env::current_dir().unwrap();
	    path_buff.push("config");
	    path_buff.push("block");
	    path_buff.set_extension("json");
	    setting.merge(config::File::from(path_buff)).unwrap();
        setting
	});

	pub static ref NETWORK_SETTINGS: RwLock<config::Config> = RwLock::new({
	    let mut setting = config::Config::default();
	    let mut path_buff = env::current_dir().unwrap();
	    path_buff.push("config");
	    path_buff.push("network");
	    path_buff.set_extension("json");
	    setting.merge(config::File::from(path_buff)).unwrap();
        setting
	});
}

