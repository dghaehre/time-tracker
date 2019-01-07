/// Display info to user


use projects::Db;

pub fn list(db: Db) {
    let projects = db.get_projects();
    match projects {
        Some(p) => println!("Found projects"),
        None    => println!("Did not find any projects")
    }
}
