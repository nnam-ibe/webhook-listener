#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use repo::{get_repo, rebuild_container, update_repo};
use rocket::serde::{json::Json, Deserialize};
mod repo;

#[derive(Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
struct Repository {
    full_name: String,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
struct PullRequest {
    merged: bool,
}

#[derive(PartialEq, Eq, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Event<'r> {
    action: &'r str,
    pull_request: PullRequest,
    repository: Repository,
}

#[get("/")]
fn index() -> &'static str {
    "Ping!"
}

#[post("/hook", format = "json", data = "<event>")]
fn hook(event: Json<Event<'_>>) -> &str {
    println!("Event received!");
    let event = event.into_inner();
    if event.action != "closed" {
        println!("Not closed event!");
        return "Action not relevant";
    }

    if !event.pull_request.merged {
        println!("Not merged event!");
        return "pull request not merged";
    }

    println!("A merged event!");
    let repo = match get_repo(event.repository.full_name) {
        Some(i) => i,
        None => {
            panic!("Unknown repo!");
        }
    };

    update_repo(&repo);
    rebuild_container(&repo);

    "Done"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let repos = std::env::var("REPOS").expect("REPOS must be set");
    let dirs = std::env::var("DIRS").expect("DIRS must be set");

    let repos_vector: Vec<String> = repos.split(',').map(|s| s.to_string()).collect();
    let dirs_vector: Vec<String> = dirs.split(',').map(|s| s.to_string()).collect();

    if repos_vector.len() != dirs_vector.len() {
        panic!("Env vars must be of the same length");
    }

    rocket::build().mount("/", routes![index, hook])
}
