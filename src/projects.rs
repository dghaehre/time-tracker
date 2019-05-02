extern crate chrono;
/// Get projects saved in ~/.time-tracker/projects.json
/// and write to file
extern crate dirs;
extern crate serde;
extern crate serde_json;

use projects::chrono::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

use display::display_error;
use display::display_status;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ProjectError {
    NoFile,
    CreateFile,
    CreateDir,
    CreateProject,
    DeleteProject,
    ParseFile,
    StartRecording,
    NoName,
    WrongName,
}

#[derive(Debug)]
pub enum FileOperation {
    Add,
    Update,
    Delete,
}

pub enum ProjectStatus {
    CreatingFile,
    ProjectCreated,
    ProjectDeleted,
}

#[derive(Clone)]
pub struct Db {
    pub name: Option<String>,
    pub jobname: Option<String>,
    pub current: Option<Project>,
    pub projects: Vec<Project>,
}

pub struct WeekdayData {
    pub data: PeriodData,
    pub weekday: Weekday,
}

pub type Weekdays = Vec<WeekdayData>;

/// (total_sec, total_amount, jobs)
pub type PeriodData = (u64, usize, Vec<Job>);

impl Db {
    /// Check for projects file
    /// if no file, create one
    /// else parse file
    pub fn init(name: Option<&str>, job_name: Option<&str>) -> Self {
        let f: Option<String> = match get_file() {
            Ok(s) => Some(s),
            Err(e) => {
                display_error(e);
                display_status(ProjectStatus::CreatingFile);
                create_file().unwrap_or_else(|e| {
                    display_error(e);
                    process::exit(1);
                })
            }
        };
        let n: Option<String> = match name {
            Some(s) => Some(s.to_owned()),
            None => None,
        };
        let jobname: Option<String> = match job_name {
            Some(s) => Some(s.to_owned()),
            None => None,
        };
        let projects = match parse_file(f) {
            Some(a) => a,
            None => {
                display_error(ProjectError::ParseFile);
                process::exit(1);
            }
        };
        let current: Option<Project> = match name {
            Some(n) => projects.iter().cloned().filter(|p| p.title == n).nth(0),
            None => None,
        };
        Db {
            name: n,
            current,
            projects,
            jobname,
        }
    }
    /// Create a new project
    pub fn new(self) -> Result<(), ProjectError> {
        match self.name {
            Some(name) => {
                let project = Project {
                    title: name,
                    description: "".to_owned(),
                    jobs: vec![],
                };
                match update_file(self.projects, project, FileOperation::Add) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            None => Err(ProjectError::NoName),
        }
    }
    pub fn delete(self) -> Result<(), ProjectError> {
        match self.name {
            Some(name) => {
                // Creating "fake" project to match against when deleting
                let project = Project {
                    title: name,
                    description: "".to_owned(),
                    jobs: vec![],
                };
                match update_file(self.projects, project, FileOperation::Delete) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            None => Err(ProjectError::NoName),
        }
    }
    /// Save new job to db
    /// and write to file
    ///
    /// Create updated project and
    /// run update_file()
    ///
    /// Init db before save incase .json has
    /// changed since first init
    pub fn save<'a>(&self, name: &str, time: u64) -> Result<(), ()> {
        let jobname: Option<&str> = match self.jobname {
            Some(ref s) => Some(&s),
            None => None,
        };
        let db = Db::init(Some(name), jobname); // Init db before save
        match db
            .projects
            .iter()
            .cloned()
            .filter(|p| p.title == name)
            .nth(0)
        {
            Some(mut project) => {
                project.add_new_job(time, db.jobname.clone());
                match update_file(self.projects.clone(), project, FileOperation::Update) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            None => Err(()),
        }
    }
    /// Fetch respective project
    /// from ~/.time-tracker-projects
    pub fn get_project(&self) -> Result<Option<Project>, ProjectError> {
        let _file = get_file()?;
        Ok(self.current.clone())
    }
    /// Get Some(name) if user has provided name
    /// that corresponds to a project stored in 'db'
    pub fn get_name(&self) -> Option<String> {
        match &self.name {
            Some(name) => {
                let exist =
                    self.projects.iter().fold(
                        false,
                        |x, project| {
                            if &project.title == name {
                                true
                            } else {
                                x
                            }
                        },
                    );
                if exist {
                    self.name.clone()
                } else {
                    None
                }
            }
            None => None,
        }
    }
    /// Fetch all projects
    /// from ~/.time-tracker-projects
    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.clone()
    }
    /// Fetch all projects
    /// that has saved any data for today
    pub fn today(&self) -> Vec<Project> {
        self.projects
            .iter()
            .map(|p| (p.today(), p))
            .filter(|((sec, _, _), _p)| sec > &0)
            .map(|((_, _, _), p)| p)
            .cloned()
            .collect()
    }
    /// Fetch all projects
    /// that as saved any data for current week
    pub fn week(&self) -> Vec<Project> {
        self.projects
            .iter()
            .map(|p| (p.week(), p))
            .filter(|(weekdays, _p)| weekdays.len() > 0)
            .map(|(_, p)| p)
            .cloned()
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub jobs: Vec<Job>,
}

impl Project {
    /// Add job for project
    fn add_new_job(&mut self, sec: u64, name: Option<String>) {
        let end: i64 = Local::now().timestamp();
        self.jobs.push(Job {
            name: name.unwrap_or("".to_owned()),
            time: Time { sec, end },
        });
    }
    /// Display jobs as stats
    /// for display <project>
    fn _get(&self) {
        unimplemented!()
    }
    /// Return (total sec, amount of jobs done)
    pub fn today(&self) -> (u64, usize, Vec<Job>) {
        let today: Vec<Job> = self
            .jobs
            .iter()
            .cloned()
            .filter(|j| {
                let job_done: NaiveDate = NaiveDateTime::from_timestamp(j.time.end, 0).date();
                let now: Date<Local> = Local::today();
                let td: NaiveDate = NaiveDate::from_ymd(now.year(), now.month(), now.day());
                job_done == td
            })
            .collect();
        (total_sec(&today), today.len(), today)
    }
    /// Return (total sec, amount of jobs done)
    /// TODO: return vec with correct days..
    pub fn week(&self) -> Weekdays {
        self.jobs
            .iter()
            .cloned()
            .filter(|j| {
                let job_done: NaiveDate = NaiveDateTime::from_timestamp(j.time.end, 0).date();
                let now: Date<Local> = Local::today();
                let monday: NaiveDate =
                    NaiveDate::from_isoywd(now.year(), now.iso_week().week(), Weekday::Mon);
                job_done >= monday
            })
            .fold(vec![], create_weekday_data)
    }
    /// Return (total sec, amount of jobs done)
    pub fn alltime(&self) -> (u64, usize) {
        (total_sec(&self.jobs), self.jobs.len())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    pub name: String,
    pub time: Time,
}

impl Job {
    pub fn add_sec(&mut self, sec: u64) {
        self.time.sec = self.time.sec + sec;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Time {
    pub sec: u64,
    end: i64,
}

fn get_full_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".time-tracker/projects.json");
    path
}

fn get_dir_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".time-tracker");
    path
}

/// Get contents from ~/time-tracker/projects.json
fn get_file() -> Result<String, ProjectError> {
    match File::open(get_full_path()) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(contents)
        }
        Err(_) => Err(ProjectError::NoFile),
    }
}

/// create ~/time-tracker
fn create_dir() -> Result<(), ProjectError> {
    match fs::create_dir(get_dir_path()) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?}", e);
            Err(ProjectError::CreateDir)
        }
    }
}

/// create ~/time-tracker/projects.json?
fn create_file() -> Result<Option<String>, ProjectError> {
    create_dir()?; // TODO dir might already exist..
    match File::create(get_full_path()) {
        Ok(mut file) => match file.write_all(b"[]") {
            Ok(_) => Ok(Some("[]".to_owned())),
            Err(_) => Err(ProjectError::CreateFile),
        },
        Err(_) => Err(ProjectError::CreateFile),
    }
}

fn parse_file(file: Option<String>) -> Option<Vec<Project>> {
    let s: String = file.unwrap_or("".to_owned());
    match serde_json::from_str(&s[..]) {
        Ok(p) => Some(p),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

/// concat job to weekday
fn create_weekday_data(mut w: Weekdays, j: Job) -> Weekdays {
    let weekday: Weekday = NaiveDateTime::from_timestamp(j.time.end, 0)
        .date()
        .weekday();
    let exist = w.iter().filter(|d| d.weekday == weekday).count() > 0;
    if exist {
        w = w.into_iter().map(|d| d).collect();
    } else {
        w.push(WeekdayData {
            data: (total_sec(&vec![j.clone()]), 1, vec![j]),
            weekday,
        });
    }
    w
}

/// Return total sec of all jobs
fn total_sec(jobs: &Vec<Job>) -> u64 {
    jobs.iter().fold(0, |t, j| t + j.time.sec)
}

/// Update ./time-tracker/projects.json
/// takes current_file (string) and the updated project.
/// If updated projects does not exist is current_file,
/// a new project is created
pub fn update_file(
    projects: Vec<Project>,
    project: Project,
    op: FileOperation,
) -> Result<Vec<Project>, ProjectError> {
    let updated: Vec<Project> = match op {
        FileOperation::Add => {
            let mut u = projects.clone();
            u.push(project);
            u
        }
        FileOperation::Update => projects.iter().fold(vec![], |mut u, p| {
            if p.title == project.title {
                u.push(project.clone());
            } else {
                u.push(p.clone());
            }
            u
        }),
        FileOperation::Delete => projects
            .into_iter()
            .filter(|p| p.title != project.title)
            .collect(),
    };
    let json = serde_json::to_string(&updated).unwrap();
    fs::write(get_full_path(), json).expect("Unable to write file");
    Ok(updated)
}
