use dotenv::dotenv;
use std::process::Command;

pub struct Repo {
    pub name: String,
    pub path: String,
}

//TODO: repos_vector & dirs_vector should be passed in as an argument
pub fn get_repo(name: String) -> Option<Repo> {
    dotenv().ok();

    let repos = std::env::var("REPOS").expect("REPOS must be set");
    let dirs = std::env::var("DIRS").expect("DIRS must be set");

    let repos_vector: Vec<String> = repos.split(',').map(|s| s.to_string()).collect();
    let dirs_vector: Vec<String> = dirs.split(',').map(|s| s.to_string()).collect();

    let index = repos_vector.iter().position(|r| r == &name);
    match index {
        Some(i) => {
            let path = dirs_vector.get(i);
            path.map(|p| Repo {
                name,
                path: p.clone(),
            })
        }
        None => {
            println!("Unknown repo");
            None
        }
    }
}

pub fn update_repo(repo: &Repo) {
    let mut command = Command::new("git");
    command.arg("pull");
    command.current_dir(repo.path.clone());

    let out = command.output().expect("Command failed");
    let data = match out.status.success() {
        true => out.stdout,
        false => out.stderr,
    };

    match String::from_utf8(data) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Invalid UTF-8 sequence: {}", e),
    };
}

pub fn rebuild_image(repo: &Repo) {
    let mut command = Command::new("docker-compose");
    command.args(["up", "-d", "--build"]);
    command.current_dir(repo.path.clone());

    let out = command.output().expect("Command failed");
    let data = match out.status.success() {
        true => out.stdout,
        false => out.stderr,
    };

    match String::from_utf8(data) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Invalid UTF-8 sequence: {}", e),
    }
}

pub fn pull_image(repo: &Repo) {
    let mut command = Command::new("docker-compose");
    command.args(["up", "-d", "--pull", "always"]);
    command.current_dir(repo.path.clone());

    let out = command.output().expect("Command failed");
    let data = match out.status.success() {
        true => out.stdout,
        false => out.stderr,
    };

    match String::from_utf8(data) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Invalid UTF-8 sequence: {}", e),
    }
}
