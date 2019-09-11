use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::fs::{OpenOptions, DirBuilder, File};
use std::path::Path;
use std::env;
use serde_json;
use core;


pub fn init_log() {
    if !Path::new(&get_home_dir()).exists() {
        DirBuilder::new()
            .recursive(false)
            .create(get_home_dir()).expect("Failed to create home dir");
    
        let mut file = File::create(get_log_file()).expect("Create file failed");
        file.write_all(b"{}").expect("Init file failed");
    }
}


pub fn read_log() -> serde_json::Value {
    let mut contents = String::new();
    read_log_as_string(&mut contents);
    serde_json::from_str(&contents).expect("Failed to parse json from file")
}


pub fn write_log(json: &serde_json::Value) {
    let file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(get_log_file()).expect("Open for write failed");
    let mut file = BufWriter::new(file);

    file.write_all(serde_json::to_string_pretty(json).unwrap().as_bytes()).expect("Write failed");
}


fn read_log_as_string(result: &mut String)  {
    let log_file = OpenOptions::new()
                    .read(true)
                    .open(get_log_file()).expect("Open for read failed");
    let mut log_file = BufReader::new(log_file);

    log_file.read_to_string(result).expect("Read from file failed");
}

fn get_log_file() -> String {
    format!("{}/{}", get_home_dir(), core::LOG_FILE)
}

fn get_home_dir() -> String {
    match env::var("LOGG_FILE") {
        Ok(val) => val,
        Err(_) => {
            let home = env::var("HOME").unwrap();
            format!("{}/{}", home, core::LOGG_HOME)
        }
    }
}


