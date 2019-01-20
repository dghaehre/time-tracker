/// Display info to user

extern crate ansi_term;

use projects::Db;
use projects::ProjectError;
use projects::ProjectStatus;
use display::ansi_term::Colour::Red;
use display::ansi_term::Colour::Green;

pub fn display_error(e: ProjectError) {
    let index = Red.bold().paint("\nERROR: ");
    match e {
        ProjectError::NoFile
            => println!("{}{}", index, Red.paint("Did not find any file in home directory\n")),
        ProjectError::CreateFile
            => println!("{}{}", index, Red.paint("Could not create file in home directory\n")),
        ProjectError::CreateDir
            => println!("{}{}", index, Red.paint("Could not create .time-tracker directory in home directory")),
        ProjectError::CreateProject
            => println!("{}{}", index, Red.paint("Could not create new project")),
        ProjectError::ParseFile
            => println!("{}{}", index, Red.paint("Could not parse file")),
        ProjectError::NoName
            => println!("{}{}", index, Red.paint("No name given"))
    }
}

pub fn display_status(s: ProjectStatus) {
    let index = Green.bold().paint("STATUS: ");
    match s {
        ProjectStatus::CreatingFile
            => println!("{}{}", index, Green.paint("Creating file in home diretory")),
        ProjectStatus::ProjectCreated
            => println!("{}{}", index, Green.paint("Project is created"))
    }
}

fn show_amount(a: usize) -> String { format!("({})\n", a.to_string()) }

pub fn list(db: Db) {
    let index = Green.bold().paint("Projects: ");
    let p = db.get_projects();
    println!("{}{}{}", index, show_amount(p.len()), "listing projects here");
}


/// Display info about <project> or if <project> is
/// not given, display summary of all projects.
pub fn stat(db: Db) {
    if db.get_name().is_some() {
        match db.get_project() {
            Ok(_)   => println!("{}", db.get_name().unwrap()),
            Err(e)  => display_error(e)
        }
    } else {
        let p = db.get_projects();
        println!("Found projects");
    }
}
