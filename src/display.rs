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
            => println!("{}{}", index, Red.paint("Could not parse file"))
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


pub fn list(db: Db) {
    match db.get_projects() {
        Ok(Some(p)) => println!("Found projects"),
        Ok(None) => println!("Did not find any projects\n\nTo create a new project:\n'time-tracker new <name>'"),
        Err(e) => display_error(e)
    }
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
        match db.get_projects() {
            Ok(Some(p)) => println!("Found projects"),
            Ok(None) => println!("Did not find any projects\n\nTo create a new project:\n'time-tracker new <name>'"),
            Err(e) => display_error(e)
        }
    }
}
