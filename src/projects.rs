/// Get projects saved in ~/.time-tracker-projects
///
///
use std::fs::File;
use std::io::prelude::*;

pub enum ProjectError {
    NoFile
}

pub struct Db {
    pub name: Option<String>,
    pub current: Option<Project>,
    pub all: Option<Vec<Project>>
}

#[derive(Clone)]
pub struct Project {
    pub title:          String,
    pub description:    String,
    pub jobs:           Vec<Job>
}

#[derive(Clone)]
pub struct Job {}

/// Get contents from ~/.time-tracker-projects
fn get_file() -> Result<String, ProjectError> {
    match File::open("Foo.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("File content {}", contents);
            Ok(contents)       
        },
        Err(_) => Err(ProjectError::NoFile)
    }
}

impl Db {
    pub fn new (name: Option<&str>) -> Self {
        let n: Option<String> = match name {
            Some(s) => Some(s.to_owned()),
            None => None
        };
        Db { name: n, current: None, all: None }
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
    pub fn get_projects(&self) -> Result<Option<Vec<Project>>, ProjectError> {
        Err(ProjectError::NoFile)
    }
    pub fn _new_project(&self, _project: Project) -> Result<String, ()> {
        Err(())
    }
    pub fn _delete(&self, _title: &str) -> Result<String, ()> {
        Err(())
    }
}

impl Project {
    /// Add job for project
    fn _add_new_job(&self, _job: Job) {
        unimplemented!()
    }
    /// Display jobs as stats
    /// for display <project>
    fn _display(&self) {
        unimplemented!()
    }
}

