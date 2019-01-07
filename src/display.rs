/// Display info to user


use projects::Db;

pub fn list(db: Db) {
    match db.get_projects() {
        Ok(Some(p)) => println!("Found projects"),
        Ok(None) => println!("Did not find any projects\n\nTo create a new project:\n'time-tracker new <name>'"),
        Err(_) => println!("Did not find any file\n\nTo create a file and a new project:\n'time-tracker new <name>'")
    }
}
