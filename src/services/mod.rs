mod jenkins;
mod nginx;
mod npm;

use jenkins::Jenkins;
use nginx::Nginx;
use npm::Npm;

pub enum Service {
    Nginx(Nginx),
    Jenkins(Jenkins),
    Npm(Npm),
}

impl Service {
    pub fn get_dockerfile(&self) -> Dockerfile {
        match self {
            Service::Nginx(nginx) => nginx.dockerfile.clone(),
            Service::Jenkins(jenkins) => jenkins.dockerfile.clone(),
            Service::Npm(npm) => npm.dockerfile.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dockerservice {
    service_name: String,
}

#[derive(Debug, Clone)]
pub struct Dockerfile {
    pub content: String,
    pub filename: String,
}

pub fn get_services() -> Vec<Service> {
    vec![
        Service::Nginx(Nginx::new()),
        Service::Jenkins(Jenkins::new()),
        Service::Npm(Npm::new()),
    ]
}
