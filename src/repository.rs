use regex::Regex;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::time::Duration;

lazy_static! {
    static ref TIMEOUT: Duration = Duration::from_secs(30);
    static ref RELEASE_RE: &'static str = r#"releases/tag/(?P<latest>\S+)""#;
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Status {
    Latest(String),
    NoRelease,
}

impl Status {
    fn status(&self) -> String {
        match self {
            Self::Latest(latest) => latest.to_string(),
            Self::NoRelease => String::from("NO RELEASE"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Repository {
    name: String,
    repository: String,
    local: String,
    status: String,
}

impl Repository {
    #[allow(dead_code)]
    fn new(name: String, repository: String, local: String, _status: String) -> Repository {
        Repository {
            name,
            repository,
            local,
            status: Status::NoRelease.status(),
        }
    }

    pub fn local(&mut self) -> String {
        self.local.clone()
    }

    pub fn status(&mut self) -> String {
        self.status.clone()
    }

    async fn check_release(&mut self) -> Result<Status, reqwest::Error> {
        let re = Regex::new(&RELEASE_RE).unwrap();

        let res = get_response(&self.repository).await?.text().await?;
        let caps = re.captures(&res);
        match caps {
            Some(caps) => Ok(Status::Latest(caps["latest"].to_string())),
            None => Ok(Status::NoRelease),
        }
    }

    pub async fn sync_status(&mut self) {
        if let Ok(status) = self.check_release().await {
            self.status = status.status();

            if Status::NoRelease == status {
                self.local = Status::NoRelease.status()
            }
            match status {
                Status::NoRelease => self.local = Status::NoRelease.status(),
                Status::Latest(_) => {
                    if self.local == String::from("") {
                        self.local = String::from("NOT SYNC");
                    }
                }
            }
        }
    }
}

async fn get_response(url: &str) -> Result<Response, reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::builder()
        .timeout(*TIMEOUT)
        .build()
        .unwrap();
    let res = client.get(url).send().await?;
    Ok(res)
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<Repository>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let data: Vec<Repository> = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn write_file<P: AsRef<Path>>(path: P, data: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();

    Ok(())
}
