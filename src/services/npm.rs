use super::docker::{
    AdvancedBuildStep, BuildStep, Dockerfile, DockerfileContent, Dockerservice,
    DockerserviceContent, Environment, Networks, Ports, Volumes,
};
use indexmap::IndexMap;

pub struct Npm {
    pub docker_file: Dockerfile,
    pub docker_service: Dockerservice,
}

impl Npm {
    pub fn new() -> Self {
        Npm {
            docker_file: Dockerfile {
                file_name: "npm.Dockerfile".to_string(),
                content: DockerfileContent::base("verdaccio/verdaccio:nightly-master")
                    .finish()
                    .to_string(),
            },
            docker_service: Dockerservice {
                service_name: "npm".into(),
                content: DockerserviceContent {
                    build_: Some(BuildStep::Advanced(AdvancedBuildStep {
                        context: ".".into(),
                        dockerfile: Some("npm.Dockerfile".into()),
                        ..Default::default()
                    })),
                    image: Some("npm:wizard".into()),
                    container_name: Some("nginx:wizard".into()),
                    ports: Ports::Short(vec!["4873:4873".into()]),
                    volumes: Volumes::Simple(vec![
                        "/home/fe_service/storage:/verdaccio/storage".into(),
                        "/home/fe_service/conf:/verdaccio/conf".into(),
                        "/home/fe_service/plugins:/verdaccio/plugins".into(),
                    ]),
                    restart: Some("always".into()),
                    environment: Environment::KvPair(IndexMap::new()),
                    networks: Networks::Simple(vec!["fe_service".into()]),
                    ..Default::default()
                },
            },
        }
    }
}
