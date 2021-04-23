#[macro_use]
extern crate lazy_static;

use std::io;
use std::process;
use std::time::Duration;

mod network;
mod repository;

lazy_static! {
    static ref REPOSITORY_JSON: &'static str = "./repos.json";
    static ref ADDR: &'static str = "www.github.com:80";
    static ref TIMEOUT: Duration = Duration::from_secs(30);
}

#[tokio::main]
async fn main() {
    if let Err(_) = network::check_network(*ADDR, *TIMEOUT).await {
        println!("Please check your network connection.");
        process::exit(1);
    }
    let repos: Vec<repository::Repository> = repository::read_file(*REPOSITORY_JSON).unwrap();

    let mut latest_repos: Vec<repository::Repository> = Vec::new();
    for mut repo in repos {
        repo.sync_status().await;
        latest_repos.push(repo);
    }

    let serialized_pretty = serde_json::to_string_pretty(&latest_repos).unwrap();
    repository::write_file(*REPOSITORY_JSON, serialized_pretty).unwrap();

    println!("List of updatable repositories:");
    for mut repo in latest_repos {
        if repo.status() != repo.local() {
            let serialized_pretty = serde_json::to_string_pretty(&repo).unwrap();
            println!("Repository {}", serialized_pretty);
        }
    }

    println!("Please press enter to quit ...");
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();
}
