//! Main binary application for bujo CLI

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::process;

mod config;
use crate::config::Config;

mod data;
use crate::data::Data;

mod display;
use crate::display::Printer;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize bujo app and create .bujorc"))
        .subcommand(SubCommand::with_name("clean").about("Remove .bujorc and .bujo directory"))
        .subcommand(SubCommand::with_name("print").about("Print all bujo objects"))
        .subcommand(SubCommand::with_name("debug").about("Print raw json data"))
        .subcommand(
            App::new("daily")
                .about("Perform actions on the daily view")
                .subcommand(
                    SubCommand::with_name("complete")
                        .about("Completes a task")
                        .arg(Arg::with_name("id").takes_value(true)),
                )
                .subcommand(
                    SubCommand::with_name("schedule")
                        .about("Schedules a task")
                        .arg(Arg::with_name("id").takes_value(true))
                        .arg(Arg::with_name("date").takes_value(true).requires("id")),
                )
                .subcommand(
                    SubCommand::with_name("subtask")
                        .arg(Arg::with_name("id").takes_value(true))
                        .about("Add a subtask to a task")
                        .arg(
                            Arg::with_name("task_description")
                                .value_name("task")
                                .help("This is the actual entry to be added to the bujo")
                                .multiple(true)
                                .requires("id"),
                        )
                        .arg(
                            Arg::with_name("task")
                                .takes_value(false)
                                .short("t")
                                .help("task flag")
                                .requires("task_description")
                                .conflicts_with("note")
                                .conflicts_with("event"),
                        )
                        .arg(
                            Arg::with_name("note")
                                .takes_value(false)
                                .short("n")
                                .help("note flag")
                                .requires("task_description")
                                .conflicts_with("task")
                                .conflicts_with("event"),
                        )
                        .arg(
                            Arg::with_name("event")
                                .takes_value(false)
                                .short("e")
                                .help("event flag")
                                .requires("task_description")
                                .conflicts_with("note")
                                .conflicts_with("task"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add entry to bujo")
                .arg(
                    Arg::with_name("task_description")
                        .value_name("task")
                        .help("This is the actual entry to be added to the bujo")
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("task")
                        .takes_value(false)
                        .short("t")
                        .help("task flag")
                        .requires("task_description")
                        .conflicts_with("note")
                        .conflicts_with("event"),
                )
                .arg(
                    Arg::with_name("note")
                        .takes_value(false)
                        .short("n")
                        .help("note flag")
                        .requires("task_description")
                        .conflicts_with("task")
                        .conflicts_with("event"),
                )
                .arg(
                    Arg::with_name("event")
                        .takes_value(false)
                        .short("e")
                        .help("event flag")
                        .requires("task_description")
                        .conflicts_with("note")
                        .conflicts_with("task"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete from raw json using HashMap id")
                .arg(Arg::with_name("id").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("migrate").about("Migrate all uncompleted tasks to today"),
        )
        .get_matches();

    let config: Config = Config::new();
    let data = Data::new(&config.data_dir);

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
        ("migrate", _) => {
            data.read().migrate_objects().write();
        }
        ("print", _) => {
            Printer::new(data).all();
        }
        ("debug", _) => {
            Printer::new(data).raw();
        }
        ("daily", Some(daily_app)) => match daily_app.subcommand() {
            ("complete", Some(complete_subcommand)) => {
                let id = complete_subcommand
                    .value_of("id")
                    .expect("Must enter daily id number");

                data.read()
                    .complete_object(id.to_string().parse().unwrap(), String::from("daily"))
                    .write();
            }
            ("schedule", Some(schedule_subcommand)) => {
                let id = schedule_subcommand
                    .value_of("id")
                    .expect("Must enter daily id number");
                let date = schedule_subcommand
                    .value_of("date")
                    .expect("Must enter date after id");
                data.read()
                    .schedule_object(
                        id.to_string().parse().unwrap(),
                        String::from("daily"),
                        date.to_string(),
                    )
                    .write();
            }
            ("subtask", Some(subtask_subcommand)) => {
                let id = subtask_subcommand
                    .value_of("id")
                    .expect("Must enter daily id number");
                let x: Vec<&str> = subtask_subcommand
                    .values_of("task_description")
                    .expect("Failed at add:task_description")
                    .collect();

                let content_type;
                if subtask_subcommand.is_present("event") {
                    content_type = String::from("event");
                } else if subtask_subcommand.is_present("note") {
                    content_type = String::from("note");
                } else {
                    content_type = String::from("task");
                }

            data.read()
                .add_subtask(id.to_string().parse().unwrap(),String::from("daily"),x.join(" ").to_string(), content_type)
                .write();
            }

            _ => Printer::new(data).daily(),
        },
        ("add", Some(sub)) => {
            let x: Vec<&str> = sub
                .values_of("task_description")
                .expect("Failed at add:task_description")
                .collect();

            let content_type;
            if sub.is_present("event") {
                content_type = String::from("event");
            } else if sub.is_present("note") {
                content_type = String::from("note");
            } else {
                content_type = String::from("task");
            }

            data.read()
                .add_object(x.join(" ").to_string(), content_type)
                .write();
        }
        (_, None) => println!("No argument provided"),
        _ => unreachable!(),
    }
}
