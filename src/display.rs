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

fn format_time(time: u64) -> String {
    if time > 9 {
        time.to_string()
    } else {
        format!("0{}", time.to_string())
    }
}

fn show_time(time: u64) -> String {
    let mut r = "".to_owned();
    let tminutes = time / 60;
    let thours = tminutes / 60;
    let h = format!("{}.", format_time(thours));
    let m = format!("{}.", format_time(tminutes - thours * 60));
    let s = format!("{}", format_time(time - tminutes * 60));
    r.push_str(&h);
    r.push_str(&m);
    r.push_str(&s);
    r
}

pub fn show_counter(name: &str, time: u64) {
    println!("{}{}   {}\n\n{}", "Working ", Green.bold().paint(name), Yellow.paint(show_time(time)), "Press ctrl-C to save current job");
}

pub fn saving(name: &str, time: u64) {
    println!("Saving work for {}\n\nSeconds: {}", Green.bold().paint(name), Yellow.paint(show_time(time)));
}

pub fn saved(r: Result<(), ()>) {
    match r {
        Ok(_)   => println!("\n{}", Green.bold().paint("Job saved succesfully")),
        Err(_)  =>  println!("\n{}", Red.bold().paint("Saving failed"))
    }
}
