//! Main binary application for bujo CLI

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};
use std::process;

mod config;
use crate::config::Config;

mod data;
use crate::data::Data;

struct TaskApp {}

impl TaskApp {
    fn new() -> App<'static, 'static> {
        App::new("task").about("Task Functions").subcommand(
            SubCommand::with_name("add").about("Add a task").arg(
                Arg::with_name("task_description")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .multiple(true),
            ),
        )
    }

    fn process_matches(matches: &ArgMatches, data: &mut Data) {
        match matches.subcommand() {
            ("add", Some(sub_m)) => TaskApp::process_add(sub_m, data),
            (_, None) => println!("Failed at task:add"),
            _ => unreachable!(),
        }
    }

    fn process_add(matches: &ArgMatches, data: &mut Data) {
        let x: Vec<&str> = matches.values_of("task_description").expect("Failed at task:add:task_descriotion").collect();
        data.add_object(x.join(" ").to_string(), String::from("task"));
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize bujo app and create .bujorc"))
        .subcommand(SubCommand::with_name("clean").about("Remove .bujorc and .bujo directory"))
        .subcommand(SubCommand::with_name("print").about("Print raw json data"))
        .subcommand(TaskApp::new())
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete from raw json using HashMap id")
                .arg(Arg::with_name("id").takes_value(true)),
        )
        .get_matches();

    let config: Config = Config::new();
    let mut data = Data::read(&config.data_dir);

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
            data.delete_object(&id);
        }
        ("print", _) => {
            for c in data.content.iter() {
                println!("{:?}", c);
            }
        }
        ("task", Some(sub_m)) => TaskApp::process_matches(sub_m, &mut data),
        (_, None) => println!("No argument provided"),
        _ => unreachable!(),
    }
    data.write(&config.data_dir);
}
