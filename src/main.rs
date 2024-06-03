#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use repo::{get_repo, pull_image, rebuild_image, update_repo};
use rocket::serde::{json::Json, Deserialize};
use rocket::tokio::spawn;
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

#[derive(PartialEq, Eq, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ImageUpdate<'r> {
    repo_name: &'r str,
}

#[get("/")]
fn index() -> &'static str {
    "Ping!"
}

#[post("/hook", format = "json", data = "<event>")]
fn hook(event: Json<Event<'_>>) {
    println!("Event received!");
    let event = event.into_inner();
    if event.action != "closed" {
        println!("Not closed event!");
        return;
    }

    if !event.pull_request.merged {
        println!("Not merged event!");
        return;
    }

    println!("A merged event!");

    if let Some(repo) = get_repo(event.repository.full_name) {
        println!("Known repo, updating...");
        spawn(async move {
            update_repo(&repo);
            rebuild_image(&repo);
        });
    }
}

#[post("/image/update", format = "json", data = "<data>")]
fn image_update(data: Json<ImageUpdate<'_>>) {
    let repo_name = data.into_inner().repo_name;
    if let Some(repo) = get_repo(repo_name.into()) {
        println!("Boy we gat a stew going, {}", repo.path);
        spawn(async move {
            pull_image(&repo);
        });
    }
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

    rocket::build().mount("/", routes![index, hook, image_update])
}
