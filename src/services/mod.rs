mod electron_builder;
mod jenkins;
mod nginx;
mod npm;

pub mod docker {
    pub use super::Dockerfile;
    pub use super::Dockerservice;
    pub use docker_compose_types::Service as DockerserviceContent;
    pub use docker_compose_types::*;
    pub use dockerfile::Dockerfile as DockerfileContent;
    pub use dockerfile::{Add, Arg, Cmd, Copy, Env, Expose, From, Label, Run, User};
}

pub use electron_builder::ElectronBuilder;
pub use jenkins::Jenkins;
pub use nginx::Nginx;
pub use npm::Npm;

use docker::DockerserviceContent;
#[derive(Clone)]
pub struct Dockerfile {
    pub file_name: String,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub struct Dockerservice {
    pub service_name: String,
    pub content: DockerserviceContent,
}

pub enum Service {
    Nginx(Nginx),
    Jenkins(Jenkins),
    Npm(Npm),
    ElectronBuilder(ElectronBuilder),
}

impl Service {
    pub fn get_docker_file(&self) -> Dockerfile {
        match self {
            Service::Nginx(nginx) => nginx.docker_file.clone(),
            Service::Jenkins(jenkins) => jenkins.docker_file.clone(),
            Service::Npm(npm) => npm.docker_file.clone(),
            Service::ElectronBuilder(electron_builder) => electron_builder.docker_file.clone(),
        }
    }

    pub fn get_docker_service(&self) -> Dockerservice {
        match self {
            Service::Nginx(nginx) => nginx.docker_service.clone(),
            Service::Jenkins(jenkins) => jenkins.docker_service.clone(),
            Service::Npm(npm) => npm.docker_service.clone(),
            Service::ElectronBuilder(electron_builder) => electron_builder.docker_service.clone(),
        }
    }

    pub fn get_config_folder(&self) -> Vec<String> {
        match self {
            Service::Nginx(nginx) => nginx.config_folders.clone().unwrap_or(vec![]),
            Service::Jenkins(jenkins) => jenkins.config_folders.clone().unwrap_or(vec![]),
            Service::Npm(npm) => npm.config_folders.clone().unwrap_or(vec![]),
            Service::ElectronBuilder(electron_builder) => {
                electron_builder.config_folders.clone().unwrap_or(vec![])
            }
        }
    }
}
