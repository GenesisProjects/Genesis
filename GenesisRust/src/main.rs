extern crate gen_core;
extern crate gen_utils;
use std::env;

fn main() {
    gen_utils::log_writer::LOGGER.write().unwrap().debug("sss", "sss", gen_utils::log_writer::LogLevel::HIGH);
}