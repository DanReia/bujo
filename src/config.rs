use home::home_dir;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub home: PathBuf,
    pub bujorc: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        let home_temp: PathBuf = home_dir().expect("Could not find home directory!");
        let bujo_temp: PathBuf = [home_temp.to_str().unwrap(), ".bujorc"].iter().collect();

        let config: Config = match fs::read_to_string(&bujo_temp) {
            Ok(file) => serde_json::from_str(&file.to_string()).unwrap(),
            Err(_) => Config {
                home: home_temp,
                bujorc: bujo_temp,
            },
        };
        config
    }

    pub fn initialize(&self) {
        if self.bujorc.exists() == true {
            println!(".bujorc already exists");
        } else {
            let bujo = Config {
                home: self.home.clone(),
                bujorc: self.bujorc.clone(),
            };
            let default = serde_json::to_string(&bujo).unwrap();
            fs::write(&self.bujorc, default).unwrap();
            println!("Created .bujorc at: {:#?}", self.bujorc);
        }
    }

    pub fn clean(&self) {
        match fs::remove_file(&self.bujorc) {
            Ok(_) => println!("Deleted .bujorc"),
            Err(_) => println!("No .bujorc to delete"),
        }
    }
}
