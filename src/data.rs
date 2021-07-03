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
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub content: HashMap<i64, BujoObject>,
    pub data_dir: PathBuf,
}

impl Data {
    pub fn new(bujo_data_dir: &PathBuf) -> Data {
        Data {
            content: HashMap::new(),
            data_dir: bujo_data_dir.to_path_buf(),
        }
    }

    /// Method to read data from file.
    /// The method always reads a file called data.json in the directory specified in the .bujorc.
    pub fn read(self) -> Data {
        let data_file: PathBuf = [self.data_dir.to_str().unwrap(), "data.json"]
            .iter()
            .collect();

        let file = match fs::read_to_string(&data_file) {
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

    /// Get max daily id
    fn get_max_daily_id(&self) -> i64 {
        let daily_ids: Vec<i64> = self.content.iter().map(|x| x.1.daily_id).collect();
        let max_val = match daily_ids.iter().max() {
            Some(x) => *x,
            None => i64::from(0),
        };
        max_val
    }

    /// Add an object to the Data HashMap. The idea is to only provide what is needed each time.
    /// This will need to be modified alot going forward
    pub fn add_object(&mut self, content_temp: String, content_type_temp: String) -> &mut Data {
        let key = self.get_max_key() + 1;
        let daily_key = self.get_max_daily_id() + 1;

        let content_t;
        let sig;
        if content_type_temp == "note" {
            content_t = String::from("note");
            sig = Signifier::Note.value();
        } else if content_type_temp == "event" {
            content_t = String::from("event");
            sig = Signifier::Event.value();
        } else {
            content_t = String::from("task");
            sig = Signifier::Task.value();
        }

        let date = Local::now().timestamp();

        let obj = BujoObject {
            content: content_temp,
            content_type: content_t,
            signifier: sig,
            current_date: date,
            date_added: date,
            daily_id: daily_key,
        };
        self.content.insert(key, obj);
        self
    }

    ///Method to get the primary key based on one of the secondary keys
    fn get_primary_key(&mut self, id: &i64, id_type: String) -> i64 {
        if id_type == String::from("daily") {
            let key = self
                .content
                .iter_mut()
                .find_map(|(key, val)| if val.daily_id == *id { Some(key) } else { None })
                .expect("Could not find key to schedule");
            *key
        } else {
            *id
        }
    }

    /// Method to schedule the given object
    /// Currently it accepts a data but converts to a datetime so that the user is not burdened
    /// with inserting the time manually. The implementation is currently very basic and only
    /// supports "%Y%m%d"
    ///
    /// At some stage it would be nice if the method left a copy of the original task with a
    /// signifier.
    pub fn schedule_object(mut self, id: i64, id_type: String, date_string: String) -> Data {
        let key = self.get_primary_key(&id, id_type);
        let naive_date = NaiveDate::parse_from_str(&date_string, "%Y%m%d").unwrap();
        let naive_time = NaiveTime::from_hms_milli(0, 0, 0, 0);
        let naive_dt = NaiveDateTime::new(naive_date, naive_time);

        match self.content.get_mut(&key) {
            Some(x) => x.current_date = naive_dt.timestamp(),
            None => println!("Could not set date"),
        }
        self
    }

    ///Method to migrate objects to today's date. 
    ///In the future we can make this more complex, however for now it moves all uncompleted tasks
    ///to the current day.
    pub fn migrate_objects(mut self) ->Data {
        let today_t = Local::now().timestamp();
        let _:() = self.content.iter_mut().map(|x| {
            if x.1.content_type == String::from("task") && x.1.signifier==Signifier::Task.value(){
                x.1.current_date = today_t;
            }
        }).collect();
        self
    }

    pub fn complete_object(mut self, id: i64, id_type: String) -> Data {
        let _: () = self
            .content
            .iter_mut()
            .map(|x| {
                if id_type == String::from("daily") {
                    if x.1.daily_id == id {
                        x.1.signifier = Signifier::Complete.value();
                    }
                }
            })
            .collect();
        self
    }

    /// Current a method called raw_delete was added purely to delete entries from the HashMap
    /// directly. In the future this will need some sort of more complex structure for improved
    /// usage.
    pub fn delete_object(&mut self, id: &i64) -> &mut Data {
        self.content.remove(id);
        self
    }
}

pub enum Signifier {
    Complete,
    Task,
    Note,
    Event,
}

impl Signifier {
    fn value(&self) -> String {
        match *self {
            Signifier::Complete => String::from("\u{00D7}"),
            Signifier::Task => String::from("\u{00B7}"),
            Signifier::Note => String::from("-"),
            Signifier::Event => String::from("\u{25CB}"),
        }
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
    pub current_date: i64,
    pub date_added: i64,
    pub daily_id: i64,
}

// impl Clone for BujoObject {
//     fn clone(&self) -> BujoObject {
//         self
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let path: PathBuf = ["./"].iter().collect();
        let mut data = Data::new(&path);
        data.add_object(String::from("this is a test"), String::from("task"));
        let serialized = serde_json::to_string(&data).unwrap();
        println!("{:?}", serialized);

        let data_file: PathBuf = [data.data_dir.to_str().unwrap(), "data.json"]
            .iter()
            .collect();
        fs::write(&data_file, &serialized).expect("There was no data directory to write to");

        let file = fs::read_to_string(&data_file).unwrap();
        let deserialized: Data = serde_json::from_str(&file).unwrap();
        assert_eq!(data.data_dir, deserialized.data_dir);
        println!("{:?}", deserialized);
    }
}
