use chrono::*;

use std::sync::RwLock;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::path::{Path,PathBuf};
use std::env;

use config_parser::SETTINGS;

lazy_static! {
    pub static ref LOGGER: RwLock<LogWritter> = {
        let mut path_buff = env::current_dir().unwrap();
        path_buff.push("log");
        let logger = LogWritter::new(&path_buff);
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

    log_path: PathBuf,
    fs: Option<File>
}

impl LogWritter {
    pub fn enabled_debug(&mut self, on: bool) { self.debug_enabled = on; }
    pub fn enabled_info(&mut self, on: bool)  { self.info_enabled = on; }
    pub fn enabled_warn(&mut self, on: bool)  { self.warn_enabled = on; }
    pub fn enabled_error(&mut self, on: bool) { self.error_enabled = on; }

    pub fn new(log_path: &PathBuf) -> Self {
        LogWritter { debug_enabled: true, info_enabled: true, warn_enabled: true, error_enabled: true, log_path: log_path.to_owned(), fs: None }
    }

    #[inline]
    pub fn debug_with_level(&self, domain: &'static str, msg: &str, log_level: LogLevel) {
        if self.debug_enabled {
            self.append_file(domain, msg, LogType::DEBUG, log_level);
        }
    }

    #[inline]
    pub fn debug(&self, domain: &'static str, msg: &str) {
        if self.debug_enabled {
            self.append_file(domain, msg, LogType::DEBUG, LogLevel::LOW);
        }
    }

    #[inline]
    pub fn info_with_level(&self, domain: &'static str, msg: &str, log_level: LogLevel) {
        if self.info_enabled {
            self.append_file(domain, msg, LogType::INFO, log_level);
        }
    }

    #[inline]
    pub fn info(&self, domain: &'static str, msg: &str) {
        if self.info_enabled {
            self.append_file(domain, msg, LogType::INFO, LogLevel::LOW);
        }
    }

    #[inline]
    pub fn warn_with_level(&self, domain: &'static str, msg: &str, log_level: LogLevel) {
        if self.warn_enabled {
            self.append_file(domain, msg, LogType::WARN, log_level);
        }
    }

    #[inline]
    pub fn warn(&self, domain: &'static str, msg: &str) {
        if self.warn_enabled {
            self.append_file(domain, msg, LogType::WARN, LogLevel::LOW);
        }
    }

    #[inline]
    pub fn error_with_level(&self, domain: &'static str, msg: &str, log_level: LogLevel) {
        if self.error_enabled {
            self.append_file(domain, msg, LogType::ERROR, log_level);
        }
    }

    #[inline]
    pub fn error(&self, domain: &'static str, msg: &str) {
        if self.error_enabled {
            self.append_file(domain, msg, LogType::ERROR, LogLevel::LOW);
        }
    }

    #[inline]
    fn gen_format(msg: &str, log_type: LogType, log_level: LogLevel) -> String {
        let now = Utc::now();
        let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let content = format!("{} [{:?}] [{:?}] {}", time_str, log_type, log_level, msg);
        println!("{:?}", content);
        content
    }

    #[inline]
    fn append_file(&self, domain: &'static str, msg: &str, log_type: LogType, log_level: LogLevel) {
        let mut tamp_path = self.log_path.to_owned();
        tamp_path.push(domain);
        tamp_path.with_extension("log");
        let mut file = OpenOptions::new().write(true).append(true).create(true).open(tamp_path).unwrap();
        let content = LogWritter::gen_format(msg, log_type, log_level);
        println!("{}", content.clone());
        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

}