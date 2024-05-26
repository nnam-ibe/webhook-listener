#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize};

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
    event.action
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hook])
}
