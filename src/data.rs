//! The purpose of the data module is to implement the serialization and deserialization of objects
//! for storage.
//!
//! Bujo Object
//!     - content_type: "task", "note", "event"
//!     - content: the actual text
//!     - id: identifier to be used for reference in the cli
//!     - uuid: unique identifier
//!
//! Ideally this all sits in a Data struct with a HashMap for content:
//!     Data.content:
//!         - key: id
//!         - value: BujoObject
//! and methods for serialize and deserialize etc.
//!
//! To prevent the data from becoming too large, all completed tasks should go into a separate data
//! file. //TODO
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::{io, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub content: HashMap<i64, BujoObject>,
}

impl Data {
    /// Constructor method for Data
    pub fn new() -> Data {
        let data_dict = Data {
            content: HashMap::new(),
        };
        data_dict
    }

    /// Method to read data from file. Used at the start of every call to bujo.
    /// The method always reads a file called data.json in the directory specified in the .bujorc.
    /// If the .bujo_data dir does not exist yet, it will be created.
    pub fn read(bujo_data_dir: &PathBuf) -> Data {
        let data_file: PathBuf = [bujo_data_dir.to_str().unwrap(), "data.json"]
            .iter()
            .collect();
        let open_file = match fs::File::open(&data_file) {
            Ok(file) => file,
            Err(_) => {
                let data = Data::new();
                if bujo_data_dir.exists() == false {
                    let mut create_dir = String::new();
                    println!("Data directory does not exist, would you like to create it at {:?}?[y/n]",bujo_data_dir);
                    io::stdin()
                        .read_line(&mut create_dir)
                        .expect("Failed to input, mut be [y/n]");
                    println!("Answer: {}",create_dir);

                    if create_dir.trim()== "y"{
                        fs::create_dir(&bujo_data_dir).unwrap();
                        data.write(&bujo_data_dir);
                        println!("Created directory:{:?}",bujo_data_dir);
                        println!("Created database: data.json");
                    } else {
                        println!("Exiting process");
                        process::exit(0);
                    }
                };
                let file = fs::File::open(&data_file).unwrap();
                file
            }
        };
        let read_data: Data = serde_json::from_reader(open_file).unwrap();
        read_data
    }

    /// Write the data back to disk. Used at the end of every CLI call.
    pub fn write(&self, bujo_data_dir: &PathBuf) {
        let data_file: PathBuf = [bujo_data_dir.to_str().unwrap(), "data.json"]
            .iter()
            .collect();
        let json_string = serde_json::to_string(&self).unwrap();
        fs::write(data_file, json_string).expect("There was no data directory to write to");
    }

    /// Get the largest key value so that any new entry does not overwrite any
    fn get_max_key(&self) -> i64 {
        match self.content.keys().max() {
            Some(x) => *x,
            None => 0,
        }
    }

    /// Add an object to the Data HashMap. The idea is to only provide what is needed each time.
    /// This will need to be modified alot going forward
    pub fn add_object(&mut self, content_temp: String, content_type_temp: String) {
        let key = self.get_max_key() + 1;
        let obj = BujoObject {
            content: content_temp,
            content_type: content_type_temp,
            signifier: String::from("."),
        };
        self.content.insert(key, obj);
        println!("Content:{:?}\n", self.content)
    }

    /// Current a method called raw_delete was added purely to delete entries from the HashMap
    /// directly. In the future this will need some sort of more complex structure for improved
    /// usage.
    pub fn delete_object(&mut self, id: &i64) {
        self.content.remove(id);
    }
}

/// This is the main object template that will be extended for every entry in the Data HashMap.
/// The idea would be that more and more attributes are added as needed to identify what the object
/// is and where it is in the system.
#[derive(Serialize, Deserialize, Debug)]
pub struct BujoObject {
    pub content_type: String,
    pub content: String,
    pub signifier: String,
}

impl Default for BujoObject {
    fn default() -> BujoObject {
        BujoObject {
            content_type: String::from("task"),
            content: String::from("placeholder text"),
            signifier: String::from("."),
        }
    }
}
