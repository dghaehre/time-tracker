/// Display info to user

extern crate ansi_term;

use projects::Db;
use projects::ProjectError;
use projects::ProjectStatus;
use display::ansi_term::Colour::Red;
use display::ansi_term::Colour::Green;
use display::ansi_term::Colour::Yellow;

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
            => println!("{}{}", index, Red.paint("Expected second argument, but no second argument given")),
        ProjectError::DeleteProject
            => println!("{}{}", index, Red.paint("Could not delete project"))
    }
}

pub fn display_status(s: ProjectStatus) {
    let index = Green.bold().paint("STATUS: ");
    match s {
        ProjectStatus::CreatingFile
            => println!("{}{}", index, Green.paint("Creating file in home diretory")),
        ProjectStatus::ProjectCreated
            => println!("{}{}", index, Green.paint("Project is created")),
        ProjectStatus::ProjectDeleted
            => println!("{}{}", index, Green.paint("Project deleted"))
    }
}

fn show_amount(a: usize) -> String { format!("({})\n", a.to_string()) }

pub fn list(db: Db) {
    let index = Green.bold().paint("Projects: ");
    let projects = db.get_projects();
    let list = projects.iter().fold("".to_owned(), |s, p| {
        s + 
        &p.title.clone() + "\n" +
        "-----------------------\n" +
        "Times tracked: " + &show_amount(p.jobs.len()) +
        "Total hours: 0\n\n\n"
    });
    println!("{}{}\n{}", index, show_amount(projects.len()), list);
}


/// Display info about <project> or if <project> is
/// not given, display summary of all projects as
/// list
pub fn stat(db: Db) {
    if db.get_name().is_some() {
        match db.get_project() {
            Ok(_)   => println!("{}", db.get_name().unwrap()),
            Err(e)  => display_error(e)
        }
    } else {
        list(db);
    }
}

pub fn show_counter(name: &str, time: u64) {
    println!("{}{}   {}\n\n{}", "Working ", Green.bold().paint(name), Yellow.paint(time.to_string()), "Press ctrl-C to save/quit current job");
}

pub fn saving(name: &str, time: u64) {
    println!("Saving work for {}\n\nSeconds: {}", Green.bold().paint(name), Yellow.paint(time.to_string()));
}
