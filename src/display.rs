/// Display info to user
extern crate ansi_term;

use display::ansi_term::Colour::Green;
use display::ansi_term::Colour::Red;
use display::ansi_term::Colour::Yellow;
use projects::Db;
use projects::Job;
use projects::Project;
use projects::ProjectError;
use projects::ProjectStatus;

pub fn display_error(e: ProjectError) {
    let index = Red.bold().paint("\nERROR: ");
    match e {
        ProjectError::NoFile => println!(
            "{}{}",
            index,
            Red.paint("Did not find any file in home directory\n")
        ),
        ProjectError::CreateFile => println!(
            "{}{}",
            index,
            Red.paint("Could not create file in home directory\n")
        ),
        ProjectError::CreateDir => println!(
            "{}{}",
            index,
            Red.paint("Could not create .time-tracker directory in home directory")
        ),
        ProjectError::CreateProject => {
            println!("{}{}", index, Red.paint("Could not create new project"))
        }
        ProjectError::ParseFile => println!("{}{}", index, Red.paint("Could not parse file")),
        ProjectError::NoName => println!(
            "{}{}",
            index,
            Red.paint("Expected second argument, but no second argument given")
        ),
        ProjectError::WrongName => println!(
            "{}{}",
            index,
            Red.paint("Second argument does not corresbond to stored projects")
        ),
        ProjectError::DeleteProject => {
            println!("{}{}", index, Red.paint("Could not delete project"))
        }
        ProjectError::StartRecording => println!(
            "{}{}",
            index,
            Red.paint("Could not create thread to record session")
        ),
    }
}

pub fn display_status(s: ProjectStatus) {
    let index = Green.bold().paint("STATUS: ");
    match s {
        ProjectStatus::CreatingFile => {
            println!("{}{}", index, Green.paint("Creating file in home diretory"))
        }
        ProjectStatus::ProjectCreated => println!("{}{}", index, Green.paint("Project is created")),
        ProjectStatus::ProjectDeleted => println!("{}{}", index, Green.paint("Project deleted")),
    }
}

fn show_amount(a: usize) -> String {
    format!("({})\n", a.to_string())
}

fn display_all(db: Db) {
    let index = Green.bold().paint("Projects: ");
    let projects = db.get_projects();
    let list = projects.iter().fold("".to_owned(), |s, p| {
        let (sec, len) = p.alltime();
        s + &p.title.clone()
            + "\n"
            + "--------------------------\n"
            + "Total: "
            + &show_time(sec)
            + "  "
            + &show_amount(len)
            + "\n\n"
    });
    println!("{}{}\n{}", index, show_amount(projects.len()), list);
}

pub fn list(db: Db) {
    let index = Green.bold().paint("Projects: ");
    let projects = db.get_projects();
    let list = projects
        .iter()
        .fold("".to_owned(), |s, p| s + &p.title.clone() + "\n");
    println!("{}{}{}", index, show_amount(projects.len()), list);
}

/// Display today's data for project
/// and overall history
fn display_project(p: Project) {
    let index = Green.bold().paint(&p.title);
    let (today_sec, today_amount, today_jobs) = p.today();
    let (alltime_sec, alltime_amount) = p.alltime();
    let today = format!("{} ({})", Yellow.paint(show_time(today_sec)), today_amount);
    let alltime = format!(
        "{} ({})",
        Yellow.paint(show_time(alltime_sec)),
        alltime_amount
    );
    let joblist = today_jobs
        .into_iter()
        .fold(vec![], |mut v: Vec<(Job, u64)>, j| {
            let mut has_same_name = false;
            v = v
                .into_iter()
                .map(|(mut job, a)| {
                    if job.name == j.name {
                        has_same_name = true;
                        job.add_sec(j.time.sec);
                        (job, a + 1)
                    } else {
                        (job, a)
                    }
                })
                .collect();
            if !has_same_name {
                v.push((j, 1));
            }
            v
        })
        .iter()
        .fold("".to_owned(), |s, (j, a)| {
            // a == additional data
            format!(
                "{}\n({}) {}   {}",
                s,
                a.to_string(),
                Yellow.paint(show_time(j.time.sec)),
                j.name
            )
        });
    println!(
        "{}\nAll time:  {}\n\nToday:     {}\n------------------------{}",
        index, alltime, today, joblist
    );
}
/// Display current week of data for project
/// TODO: display day by day
fn display_project_week(p: Project) {
    let index = Green.bold().paint(&p.title);
    let weekday_data = p.week();
    let (week_sec, week_amount, week_jobs) = weekday_data.get(0).unwrap().data.clone();
    let week = format!("{} ({})", Yellow.paint(show_time(week_sec)), week_amount);
    let joblist = week_jobs
        .into_iter()
        .fold(vec![], |mut v: Vec<(Job, u64)>, j| {
            let mut has_same_name = false;
            v = v
                .into_iter()
                .map(|(mut job, a)| {
                    if job.name == j.name {
                        has_same_name = true;
                        job.add_sec(j.time.sec);
                        (job, a + 1)
                    } else {
                        (job, a)
                    }
                })
                .collect();
            if !has_same_name {
                v.push((j, 1));
            }
            v
        })
        .iter()
        .fold("".to_owned(), |s, (j, a)| {
            // a == additional data
            format!(
                "{}\n({}) {}   {}",
                s,
                a.to_string(),
                Yellow.paint(show_time(j.time.sec)),
                j.name
            )
        });
    println!(
        "{}\nThis week:     {}\n------------------------{}",
        index, week, joblist
    );
}

/// Display info about <project> or if <project> is
/// not given, display summary of all projects as
/// list
pub fn stat(db: Db) {
    match db.name {
        Some(_) => match db.get_project() {
            Ok(Some(p)) => display_project(p),
            Ok(None) => display_error(ProjectError::WrongName),
            Err(e) => display_error(e),
        },
        None => display_all(db),
    }
}

/// Display info about all projects that has been
/// worked on today
pub fn today(db: Db) {
    let today = db.today();
    let index = Green.bold().paint("Projects today: ");
    println!("{} ({})", index, today.len());
    for p in today.into_iter() {
        println!(" ");
        display_project(p);
        println!("--------------------------------------------");
    }
}

/// Display info about all projects that has been
/// worked on this week
/// TODO: Not finished
pub fn week(db: Db) {
    let week = db.week();
    let index = Green.bold().paint("Projects worked on this week: ");
    println!("{} ({})", index, week.len());
    for p in week.into_iter() {
        println!(" ");
        display_project_week(p);
        println!("--------------------------------------------");
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

pub fn show_counter(name: &str, time: u64, jobname: &Option<String>) {
    let show_jobname = match jobname {
        Some(j) => format!("{}\n", Yellow.bold().paint(j)),
        None => "".to_owned(),
    };
    println!(
        "Working {}   {}\n{}\n{}",
        Green.bold().paint(name),
        Yellow.paint(show_time(time)),
        show_jobname,
        "Press ctrl-C to stop and save current job"
    );
}

pub fn saving(name: &str, time: u64) {
    println!(
        "Saving work for {}\n\nTime spent: {}",
        Green.bold().paint(name),
        Yellow.paint(show_time(time))
    );
}

pub fn saved(r: Result<(), ()>) {
    match r {
        Ok(_) => println!("\n{}", Green.bold().paint("Job saved succesfully")),
        Err(_) => println!("\n{}", Red.bold().paint("Saving failed")),
    }
}
