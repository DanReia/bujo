use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

mod config;
use crate::config::Config;

mod data;
use crate::data::{BujoObject, Data};

///Main binary application for bujo CLI
fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize bujo app and create .bujorc"))
        .subcommand(SubCommand::with_name("clean").about("Remove .bujorc and .bujo directory"))
        .subcommand(SubCommand::with_name("print").about("Print raw json data"))
        .subcommand(
            SubCommand::with_name("task")
                .about("Task Functions")
                .subcommand(
                    SubCommand::with_name("add").about("Add a task").arg(
                        Arg::with_name("task_description")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                            .multiple(true),
                    ),
                ),
        )
        .subcommand(
            SubCommand::with_name("raw_delete")
                .about("Delete from raw json using HashMap id")
                .arg(Arg::with_name("id").takes_value(true)),
        )
        .get_matches();

    let config: Config = Config::new();
    let mut data = Data::read(&config.data_dir);

    match matches.subcommand() {
        ("init", _) => config.initialize(),
        ("clean", _) => config.clean(),
        ("raw_delete", Some(sub_m)) => {
            let id: i64 = sub_m.value_of("id").unwrap().parse().unwrap();
            data.delete_object(&id);
        },
        ("print", _) => {
            println!("{:?}", data);
        }
        ("task", Some(sub_m)) => match sub_m.subcommand() {
            ("add", Some(sub_m)) => {
                let x: Vec<&str> = sub_m.values_of("task_description").unwrap().collect();
                data.add_object(x.join(" ").to_string(), String::from("task"));
            }
            (_, None) => println!("No value"),
            _ => unreachable!(),
        },

        (_, None) => println!("No argument provided"),
        _ => unreachable!(),
    }
    data.write(&config.data_dir);
}
