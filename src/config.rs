//! # Config
//! Crate to host the Configuration object for the bujo application

use home::home_dir;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub home: PathBuf,
    pub bujorc: PathBuf,
    pub data_dir: PathBuf,
}

/// The purpose of Config is to handle all references to config related items
/// such as those relating to the .bujorc as well as to the data directories.
impl Config {
    /// Config::new() is intended to instantiate the Configuration of the current
    /// application call by discovering the home directory for the operating system
    /// and adding a reference to .bujorc.
    pub fn new() -> Config {
        let home_temp: PathBuf = home_dir().expect("Could not find home directory!");
        let bujo_temp: PathBuf = [&home_temp.to_str().unwrap(), ".bujorc"].iter().collect();
        let data_dir_temp: PathBuf = [&home_temp.to_str().unwrap(), ".bujo_data"].iter().collect();

        let config: Config = match fs::read_to_string(&bujo_temp) {
            Ok(file) => serde_json::from_str(&file.to_string()).unwrap(),
            Err(_) => Config {
                home: home_temp,
                bujorc: bujo_temp,
                data_dir: data_dir_temp
            },
        };
        config
    }

    /// Config::initialize() implements the creation of a .bujorc if it does not
    /// exist yet. As well as a .bujo_data directory
    pub fn initialize(&self) {
        if self.bujorc.exists() {
            println!(".bujorc already exists");
        } else {
            let bujo = Config {
                home: self.home.clone(),
                bujorc: self.bujorc.clone(),
                data_dir: self.data_dir.clone(),
            };
            let default = serde_json::to_string(&bujo).unwrap();
            fs::write(&self.bujorc, default).unwrap();
            println!("Created .bujorc at: {:#?}", self.bujorc);
        }

        if self.data_dir.exists(){
            println!("Data directory: {:#?} already exists",self.data_dir);
        }
        else {
            fs::create_dir(&self.data_dir).unwrap();
            println!("Created .bujo_data at: {:#?}",self.data_dir);
        }
    }

    /// Config::clean() deletes the .bujorc and .bujo_data if it exists.
    pub fn clean(&self) {
        match fs::remove_file(&self.bujorc) {
            Ok(_) => println!("Deleted .bujorc"),
            Err(_) => println!("No .bujorc to delete"),
        };

        match fs::remove_dir_all(&self.data_dir) {
            Ok(_) => println!("Deleted .bujo_data"),
            Err(_) => println!("No .bujo_data to delete"),
        };
    }
}
