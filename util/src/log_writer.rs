use chrono::*;

use std::sync::RwLock;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::{File, OpenOptions};

use std::path::Path;

use config_parser::SETTINGS;

lazy_static! {
    pub static ref LOGGER: RwLock<LogWritter> = {
        let path = (SETTINGS.read().unwrap().get::<String>("logPath").unwrap());
        let mut logger = LogWritter::new(&path);
        RwLock::new(logger)
    };
}

#[derive(Debug)]
pub enum LogLevel {
    HIGH,
    MEDIUM,
    LOW
}

#[derive(Debug)]
pub enum LogType {
    DEBUG,
    INFO,
    WARN,
    ERROR
}

pub struct LogWritter {
    debug_enabled: bool,
    info_enabled: bool,
    warn_enabled: bool,
    error_enabled: bool,

    log_path: String,
    fs: Option<File>
}

impl LogWritter {
    pub fn enabled_debug(&mut self, on: bool) { self.debug_enabled = on; }
    pub fn enabled_info(&mut self, on: bool)  { self.info_enabled = on; }
    pub fn enabled_warn(&mut self, on: bool)  { self.warn_enabled = on; }
    pub fn enabled_error(&mut self, on: bool) { self.error_enabled = on; }

    pub fn new(log_path: &String) -> Self {
        LogWritter { debug_enabled: true, info_enabled: true, warn_enabled: true, error_enabled: true, log_path: log_path.to_owned(), fs: None }
    }

    #[inline]
    pub fn debug(&self, domain: &'static str, msg: &'static str, log_level: LogLevel) {
        if self.debug_enabled {

        }
    }

    #[inline]
    pub fn info(&self, domain: &'static str, msg: &'static str, log_level: LogLevel) {


        unimplemented!()
    }

    #[inline]
    pub fn warn(&self, domain: &'static str, msg: &'static str, log_level: LogLevel) {
        unimplemented!()
    }

    #[inline]
    pub fn error(&self, domain: &'static str, msg: &'static str, log_level: LogLevel) {
        unimplemented!()
    }

    #[inline]
    fn gen_format(msg: &'static str, log_type: LogType, log_level: LogLevel) -> String {
        let now = Utc::now();
        let time_str = now.format("%b %-d, %-I:%M").to_string();
        format!("{:?} [{:?}] [{:?}] {}", time_str, log_type, log_level, msg)
    }

    #[inline]
    fn append_file(&self, domain: &'static str, msg: &'static str, log_type: LogType, log_level: LogLevel) {
        let mut file = OpenOptions::new().write(true).append(true).create_new(true).open(self.log_path.to_owned() + domain + ".log").unwrap();
        let content = LogWritter::gen_format(msg, log_type, log_level);
        if let Err(e) = writeln!(file, "") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

}