/// Get projects saved in ~/.time-tracker/projects.json
/// and write to file
extern crate dirs;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::process;
use std::path::PathBuf;

use display::display_error;
use display::display_status;

#[derive(Debug)]
pub enum ProjectError {
    NoFile,
    CreateFile,
    CreateDir,
    CreateProject,
    ParseFile,
    NoName
}

pub enum ProjectStatus {
    CreatingFile,
    ProjectCreated
}

pub struct Db {
    pub name:       Option<String>,
    pub current:    Option<Project>,
    pub projects:   Vec<Project>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub title:          String,
    pub description:    String,
    pub jobs:           Vec<Job>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Job {}
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
                match update_file(self.projects, project) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            }
            None => Err(ProjectError::NoName)
        }
    }

    /// Fetch respective project
    /// from ~/.time-tracker-projects
    pub fn get_project(&self) -> Result<Option<Project>, ProjectError> {
        let _file = get_file()?;
        Ok(self.current.clone())
    }

    // To be removed!!
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
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
    
fn write_to_file(v: Vec<Project>) -> Result<(), ProjectError> {
    unimplemented!()
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
pub fn update_file(projects: Vec<Project>, project: Project)
    -> Result<Vec<Project>, ProjectError> {
        let mut updated = projects.clone();
        let exist: bool = projects.iter().fold(false, |e, p| {
                if p.title == project.title {
                    true
                } else {
                    e
                }
            });
        if exist {
            updated = updated.iter().fold(vec![], |mut u, p| {
                if p.title == project.title {
                    u.push(project.clone());
                } else {
                    u.push(p.clone());
                }
                u
            });
        } else {
           updated.push(project);
        }
        let json = serde_json::to_string(&updated).unwrap();
        fs::write(get_full_path(), json).expect("Unable to write file");
        Ok(updated)
 }


