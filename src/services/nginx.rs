
use crate::services::{Dockerfile, Dockerservice};
pub struct Nginx {
    pub dockerfile: Dockerfile,
    pub service: Dockerservice,
}

impl Nginx {
    pub fn new() -> Self {
        Nginx {
            dockerfile: Dockerfile {
                content: dockerfile::Dockerfile::base("verdaccio/verdaccio:nightly-master")
                    .finish()
                    .to_string(),
                filename: "npm.Dockerfile".to_string(),
            },
            service: Dockerservice {
                service_name: "npm".to_string(),
            },
        }
    }
}
