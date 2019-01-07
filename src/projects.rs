/// Get projects saved in ~/.time-tracker-projects
///
///

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
    pub fn get_project(&self) -> Option<Project> {
        self.current.clone()
    }
    // To be removed!!
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn get_projects(&self) -> Option<Vec<Project>> {
        self.all.clone()
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

