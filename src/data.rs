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
use std::process;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub content: HashMap<i64, BujoObject>,
    pub data_dir: PathBuf
}

impl Data {
    pub fn new(bujo_data_dir: &PathBuf) -> Data{
        Data{
            content: HashMap::new(),
            data_dir: bujo_data_dir.to_path_buf(),
        }
    }

    /// Method to read data from file.
    /// The method always reads a file called data.json in the directory specified in the .bujorc.
    pub fn read(&mut self) -> Data {
        let data_file: PathBuf = [self.data_dir.to_str().unwrap(), "data.json"]
            .iter()
            .collect();

        let file = match fs::read_to_string(&data_file){
            Ok(file) => file,
            Err(_) => {
                println!("It appears that the data directory does not exist yet. Try run `bujo init` first");
                process::exit(1);
                }
        };
        let read_file: Data = serde_json::from_str(&file).expect("Error reading file");
        read_file
    }

    /// Write the data back to disk. Used at the end of every CLI call.
    pub fn write(&self) {
        let data_file: PathBuf = [self.data_dir.to_str().unwrap(), "data.json"]
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
    pub fn add_object(&mut self, content_temp: String, content_type_temp: String) -> &mut Data{
        let key = self.get_max_key() + 1;
        let obj = BujoObject {
            content: content_temp,
            content_type: content_type_temp,
            signifier: String::from("."),
        };
        self.content.insert(key, obj);
        self
    }

    /// Current a method called raw_delete was added purely to delete entries from the HashMap
    /// directly. In the future this will need some sort of more complex structure for improved
    /// usage.
    pub fn delete_object(&mut self, id: &i64)-> &mut Data {
        self.content.remove(id);
        self
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_serde(){
        let path: PathBuf = ["./"].iter().collect();
        let mut data=Data::new(&path);
        data.add_object(String::from("this is a test"),String::from("task"));
        let serialized = serde_json::to_string(&data).unwrap();
        println!("{:?}",serialized);


        let data_file: PathBuf = [data.data_dir.to_str().unwrap(), "data.json"]
            .iter()
            .collect();
        fs::write(&data_file, &serialized).expect("There was no data directory to write to");


        let file = fs::read_to_string(&data_file).unwrap();
        let deserialized: Data = serde_json::from_str(&file).unwrap();
        assert_eq!(data.data_dir,deserialized.data_dir);
        println!("{:?}",deserialized);

    }
}
