//! Main binary application for bujo CLI

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand,
};
use std::process;

mod config;
use crate::config::Config;

mod data;
use crate::data::Data;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize bujo app and create .bujorc"))
        .subcommand(SubCommand::with_name("clean").about("Remove .bujorc and .bujo directory"))
        .subcommand(SubCommand::with_name("print").about("Print raw json data"))
        .subcommand(
            SubCommand::with_name("add").about("Add entry to bujo").arg(
                Arg::with_name("task_description")
                    .value_name("task")
                    .help("This is the actual entry to be added to the bujo")
                    .multiple(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete from raw json using HashMap id")
                .arg(Arg::with_name("id").takes_value(true)),
        )
        .get_matches();

    let config: Config = Config::new();
    let mut data = Data::new(&config.data_dir);

    match matches.subcommand() {
        ("init", _) => config.initialize(),
        ("clean", _) => {
            config.clean();
            // Need to exit cleanly out of process otherwise data.write is called
            // without a data file present
            process::exit(0)
        }
        ("delete", Some(sub_m)) => {

            let id: i64 = sub_m.value_of("id").unwrap().parse().unwrap();
            data.read().delete_object(&id).write();
        }
        ("print", _) => {
            for c in data.read().content.iter() {
                println!("{:?}", c);
            }
        }
        ("add", Some(sub)) => {
            let x: Vec<&str> = sub
                .values_of("task_description")
                .expect("Failed at add:task_description")
                .collect();
            data.read().add_object(x.join(" ").to_string(), String::from("task")).write();
        }
        (_, None) => println!("No argument provided"),
        _ => unreachable!(),
    }
}
