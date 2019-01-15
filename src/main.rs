//! This is a cli tool to simply track your time spent on different
//! projects.
//!
//! USAGE:
//! time-tracker list
//! time-tracker start <project>
//! time-tracker stop <project>
//! time-tracker new <project>
//! time-tracker delete <project>
//! time-tracker display <project>
//! trime-tracker display

extern crate clap;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App};

mod projects;
mod display;

use projects::Db;
use projects::ProjectStatus;
use display::display_error;
use display::display_status;


fn main() {

    let arg_project = Arg::with_name("PROJECT")
                            .index(2)
                            .help("Name of project");

    let command = Arg::with_name("COMMAND")
                            .index(1)
                            .required(true)
                            .help("list, start, stop, new, delete, display");

    let matches = App::new("Time tracker")
                        .version("1.0")
                        .author("Daniel Hæhre <dghaehre@gmail.com>")
                        .about("A cli tool for tracking time spent on projects")
                        .arg(command)
                        .arg(arg_project)
                        .get_matches();

    let project_name = matches.value_of("PROJECT");

    let db = Db::init(project_name);

    match matches.value_of("COMMAND").unwrap() {
        "list"      => display::list(db),
        "start"     => start(db),
        "stop"      => println!("Stop"),
        "new"       => new(db),
        "delete"    => println!("delete"),
        "display"   => display::stat(db),
        _           => println!("Command not valid.. ")
    };
}

fn new(db: Db) {
    match db.new() {
        Ok(_) => display_status(ProjectStatus::ProjectCreated),
        Err(e) => display_error(e)
    }
}

// To be moved
fn start(db: Db) {
    let project = db.get_project();
    match db.get_name() {
        Some(name) => println!("Start {}", name),
        None       => println!("Missing project name\n\nUsage:\ntime-tracker start <project>")
    }
}
