mod jenkins;
mod nginx;
mod npm;

use crate::write_to_dir;
use anyhow::Result;
use docker_compose_types::{Compose, Service, Services};
use indexmap::IndexMap;

#[derive(Debug)]
pub enum DockerService {
    Jenkins,
    Nginx,
    Npm,
}

pub fn gen_dockercompose() -> Result<()> {
    let mut services = IndexMap::new();
    let service = (
        "web".to_string(),
        Some(Service {
            image: Some("nginx:latest".to_string()),
            ..Default::default()
        }),
    );
    services.insert(service.0, service.1);

    let compose_content = Compose {
        version: Some("3.8".to_string()),
        services: { Services(services) },
        ..Default::default()
    };

    let serialized = match serde_yaml::to_string(&compose_content) {
        Ok(s) => s,
        Err(e) => panic!("Failed to serialize docker-compose file: {}", e),
    };

    write_to_dir("services", "docker-compose.yml", serialized)?;
    Ok(())
}
