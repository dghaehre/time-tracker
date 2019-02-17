/// Display info to user

extern crate ansi_term;

use projects::Project;
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
        ProjectError::WrongName
            => println!("{}{}", index, Red.paint("Second argument does not corresbond to stored projects")),
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

fn display_all(db: Db) {
    let index = Green.bold().paint("Projects: ");
    let projects = db.get_projects();
    let list = projects.iter().fold("".to_owned(), |s, p| {
        let (sec, len) = p.alltime();
        s + 
        &p.title.clone() + "\n" +
        "-----------------------\n" +
        "Total: " +
        &show_time(sec) +
        "  " +
        &show_amount(len) +
        "\n\n"
    });
    println!("{}{}\n{}", index, show_amount(projects.len()), list);
}

pub fn list(db: Db) {
    let index = Green.bold().paint("Projects: ");
    let projects = db.get_projects();
    let list = projects.iter().fold("".to_owned(), |s, p| { s + &p.title.clone() + "\n" });
    println!("{}{}{}", index, show_amount(projects.len()), list);
}

/// Display today's data for project
/// and overall history
fn display_project(p: Project) {
    let index = Green.bold().paint(&p.title);
    let (today_sec, today_amount) = p.today();
    let (alltime_sec, alltime_amount) = p.alltime();
    let today = format!("{} ({})", Yellow.paint(show_time(today_sec)), today_amount);
    let alltime = format!("{} ({})", Yellow.paint(show_time(alltime_sec)), alltime_amount);
    println!("{}\nToday:     {}\nAll time:  {}\n", index, today, alltime);
}

/// Display info about <project> or if <project> is
/// not given, display summary of all projects as
/// list
pub fn stat(db: Db) {
    match db.name {
        Some(_) => match db.get_project() {
                    Ok(Some(p)) => display_project(p),
                    Ok(None)    => display_error(ProjectError::WrongName),
                    Err(e)      => display_error(e)
        },
        None => display_all(db)
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
