use clap::{crate_authors, crate_description, crate_name, crate_version, App, SubCommand};

mod config;
use crate::config::Config;

///Main binary application for bujo CLI
fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize bujo app and create .bujorc"))
        .subcommand(SubCommand::with_name("clean").about("Remove .bujorc and .bujo directory"))
        .get_matches();
    
    let config: Config = Config::new();
    println!("home: {:#?}",config.home);
    println!("bujorc: {:#?}",config.bujorc);


    match matches.subcommand_name() {
        Some("init") => config.initialize(),
        Some("clean") => config.clean(),
        None => println!("No argument provided"),
        _ => unreachable!(),
    }
}
