//! This is a cli tool to simply track your time spent on different
//! projects.
//!
//! USAGE:
//! time-tracker list
//! time-tracker start <project>
//! time-tracker new <project>
//! time-tracker delete <project>
//! trime-tracker display
//! time-tracker display <project>

extern crate clap;
extern crate ctrlc;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App};
mod projects;
mod display;

use projects::Db;
use projects::ProjectStatus;
use projects::ProjectError;
use display::display_error;
use display::display_status;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {

    let arg_project = Arg::with_name("PROJECT")
                            .index(2)
                            .help("Name of project");

    let command = Arg::with_name("COMMAND")
                            .index(1)
                            .required(true)
                            .help("list, start, new, delete, display");

    let matches = App::new("Time tracker")
                        .version("0.1")
                        .author("Daniel Hæhre <dghaehre@gmail.com>")
                        .about("A cli tool for tracking time spent on projects")
                        .arg(command)
                        .arg(arg_project)
                        .get_matches();

    let project_name = matches.value_of("PROJECT");

    // Init time-tracker
    // If no file exist, create file in home directory
    let db = Db::init(project_name);

    match matches.value_of("COMMAND").unwrap() {
        "list"      => display::list(db),
        "start"     => start(db),
        "new"       => new(db),
        "delete"    => delete(db),
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

fn delete(db: Db) {
    match db.delete() {
        Ok(_) => display_status(ProjectStatus::ProjectDeleted),
        Err(e) => display_error(e)
    }
}

// To be moved
fn start(db: Db) {
    match db.get_name() {
        Some(_) => start_record(db),
        None       => println!("Missing project name\n\nUsage:\ntime-tracker start <project>")
    }
}

fn start_record(db: Db) {
    let name = db.get_name().unwrap();
    let now = Instant::now();
    let finished = Arc::new(AtomicBool::new(false));
    let n = Arc::new(name);
    let nn = n.clone();
    let f = finished.clone();
    let d = Arc::new(db);
    if let Err(_) = ctrlc::set_handler(move || {
            std::process::Command::new("clear").status().unwrap();
            f.store(true, Ordering::Relaxed);
            display::saving(&nn, now.elapsed().as_secs());
            display::saved(d.save(&nn, now.elapsed().as_secs()));
        }) {
        display_error(ProjectError::StartRecording);
    };
    while !finished.load(Ordering::Relaxed) {
        std::process::Command::new("clear").status().unwrap();
        display::show_counter(&n, now.elapsed().as_secs());
        thread::sleep(Duration::from_secs(1));
    }
}
