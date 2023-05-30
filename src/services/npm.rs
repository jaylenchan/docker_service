// use crate::Service;
use crate::services::{Dockerfile, Dockerservice};
use dockerfile::{Cmd, Expose, Label, Run};
pub struct Npm {
    pub docker_file: Dockerfile,
    pub docker_service: Dockerservice,
}

impl Npm {
    pub fn new() -> Self {
        Npm {
            docker_file: Dockerfile {
                content: dockerfile::Dockerfile::base("verdaccio/verdaccio:nightly-master")
                    .finish()
                    .to_string(),
                filename: "npm.Dockerfile".to_string(),
            },
            docker_service: Dockerservice {
                service_name: "npm".to_string(),
                image: "npm".to_string(),
            },
        }
    }
}
