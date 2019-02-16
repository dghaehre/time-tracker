/// Get projects saved in ~/.time-tracker/projects.json
/// and write to file
extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::process;
use std::path::PathBuf;
use projects::chrono::prelude::*;

use display::display_error;
use display::display_status;

#[derive(Debug)]
pub enum ProjectError {
    NoFile,
    CreateFile,
    CreateDir,
    CreateProject,
    DeleteProject,
    ParseFile,
    NoName
}

#[derive(Debug)]
pub enum FileOperation {
    Add,
    Update,
    Delete
}

pub enum ProjectStatus {
    CreatingFile,
    ProjectCreated,
    ProjectDeleted
}

#[derive(Clone)]
pub struct Db {
    pub name:       Option<String>,
    pub current:    Option<Project>,
    pub projects:   Vec<Project>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub title:          String,
    pub description:    String,
    pub jobs:           Vec<Job>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    pub name:   String,
    pub time:   Time
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Time {
    sec:    u64,
    end:    String
}

impl Db {
    /// Check for projects file
    /// if no file, create one
    /// else parse file
    pub fn init(name: Option<&str>) -> Self {
        let f: Option<String> = match get_file() {
            Ok(s) => Some(s),
            Err(e) => {
                display_error(e);
                display_status(ProjectStatus::CreatingFile);
                create_file()
                    .unwrap_or_else(|e| {
                        display_error(e);
                        process::exit(1);
                    })
            }
        };
        let n: Option<String> = match name {
            Some(s) => Some(s.to_owned()),
            None => None
        };
        let projects = match parse_file(f) {
            Some(a) => a,
            None    => {
                display_error(ProjectError::ParseFile);
                process::exit(1);
            }
        };
        Db { name: n, current: None, projects: projects }
    }

    /// Create a new project
    pub fn new(self) -> Result<(), ProjectError> {
        match self.name {
            Some(name) => {
                let project = Project {
                    title: name,
                    description: "".to_owned(),
                    jobs: vec![]
                };
                match update_file(self.projects, project, FileOperation::Add) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            }
            None => Err(ProjectError::NoName)
        }
    }

    pub fn delete(self) -> Result<(), ProjectError> {
        match self.name {
            Some(name) => {
                // Creating "fake" project to match against when deleting
                let project = Project {
                    title: name,
                    description: "".to_owned(),
                    jobs: vec![]
                };
                match update_file(self.projects, project, FileOperation::Delete) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            },
            None => Err(ProjectError::NoName)
        }
    }

    /// Save new job to db
    /// and write to file
    pub fn save(&self, name: &str, time: u64) -> Result<(), ()> {
        Err(())
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
                let exist = self.projects
                    .iter()
                    .fold(false, |x, project| {
                        if &project.title == name {
                            true
                        } else {
                            x
                        }
                    });
                if exist {
                    self.name.clone()
                } else {
                    None
                }
            },
            None => None
        }
    }

    /// Fetch all projects
    /// from ~/.time-tracker-projects
    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.clone()
    }
}


impl Project {
    /// Add job for project
    fn _add_new_job(&self, _job: Job) {
        unimplemented!()
    }
    /// Display jobs as stats
    /// for display <project>
    fn _get(&self) {
        unimplemented!()
    }
}




fn get_full_path() -> PathBuf {
    let mut path = dirs::home_dir()
                        .unwrap();
    path.push(".time-tracker/projects.json");
    path
}

fn get_dir_path() -> PathBuf {
    let mut path = dirs::home_dir()
                        .unwrap();
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
        },
        Err(e) => Err(ProjectError::NoFile)
    }
}

/// create ~/time-tracker
fn create_dir() -> Result<(), ProjectError> {
    match fs::create_dir(get_dir_path()) {
        Ok(_)    => Ok(()),
        Err(e)  => {
            println!("{:?}", e);
            Err(ProjectError::CreateDir)
        }
    }
}

/// create ~/time-tracker/projects.json? 
fn create_file() -> Result<Option<String>, ProjectError> {
    create_dir()?; // TODO dir might already exist..
    match File::create(get_full_path()) {
        Ok(mut file) => {
           match file.write_all(b"[]") {
               Ok(_) => Ok(Some("[]".to_owned())),
               Err(_) => Err(ProjectError::CreateFile)
           }
        },
        Err(e) => Err(ProjectError::CreateFile)
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

fn has_project(all: &Vec<Project>, new: &Project) -> bool {
    all.iter().fold(false, |x, project| {
        if project.title == new.title {
            true
        } else {
            x
        }
    })
}

fn update_project(all: &Vec<Project>, new: &Project) -> Vec<Project> {
    all.iter().fold(vec![], |mut l, project| {
        if project.title == new.title {
            l.push(new.clone());
            l
        } else {
            l.push(project.clone());
            l
        }
    })
}

/// Update ./time-tracker/projects.json
/// takes current_file (string) and the updated project.
/// If updated projects does not exist is current_file,
/// a new project is created
pub fn update_file(projects: Vec<Project>, project: Project, op: FileOperation)
    -> Result<Vec<Project>, ProjectError> {
        let updated: Vec<Project> = match op {
            FileOperation::Add => {
                let mut u = projects.clone();
                u.push(project);
                u
            },
            FileOperation::Update => {
                projects.iter().fold(vec![], |mut u, p| {
                    if p.title == project.title {
                        u.push(project.clone());
                    } else {
                        u.push(p.clone());
                    }
                    u
                })
            },
            FileOperation::Delete => {
                projects
                    .into_iter()
                    .filter(|p| p.title != project.title)
                    .collect()
            }
        };
        let json = serde_json::to_string(&updated).unwrap();
        fs::write(get_full_path(), json).expect("Unable to write file");
        Ok(updated)
}


