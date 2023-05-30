mod jenkins;
mod nginx;
mod npm;

pub use jenkins::Jenkins;
pub use nginx::Nginx;
pub use npm::Npm;

#[derive(Debug, Clone)]
pub struct Dockerservice {
    service_name: String,
    image: String,
}

#[derive(Debug, Clone)]
pub struct Dockerfile {
    pub content: String,
    pub filename: String,
}

pub enum Service {
    Nginx(Nginx),
    Jenkins(Jenkins),
    Npm(Npm),
}

impl Service {
    pub fn get_docker_file(&self) -> Dockerfile {
        match self {
            Service::Nginx(nginx) => nginx.docker_file.clone(),
            Service::Jenkins(jenkins) => jenkins.docker_file.clone(),
            Service::Npm(npm) => npm.docker_file.clone(),
        }
    }

    pub fn get_docker_service(&self) -> Dockerservice {
        match self {
            Service::Nginx(nginx) => nginx.docker_service.clone(),
            Service::Jenkins(jenkins) => jenkins.docker_service.clone(),
            Service::Npm(npm) => npm.docker_service.clone(),
        }
    }
}
