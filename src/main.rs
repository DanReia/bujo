use clap::{crate_authors, crate_description, crate_name, crate_version, App, SubCommand};

///Main binary application for bujo CLI
fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize bujo app"))
        .subcommand(SubCommand::with_name("clean").about("Remove .bujorc and .bujo directory"))
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => println!("Initialize"),
        Some("clean") => println!("clean up!"),
        None => println!("No argument provided"),
        _ => unreachable!(),
    }
}
