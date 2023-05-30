mod jenkins;
mod nginx;
mod npm;

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
    let compose_content = Compose {
        version: Some("3.8".to_string()),
        services: {
            let mut services = IndexMap::new();
            services.insert(
                "web".to_string(),
                Some(Service {
                    image: Some("nginx:latest".to_string()),
                    ..Default::default()
                }),
            );
            Services(services)
        },
        ..Default::default()
    };

    let target_file = std::path::Path::new("docker-compose.yml");
    // serialize to string
    let serialized = match serde_yaml::to_string(&compose_content) {
        Ok(s) => s,
        Err(e) => panic!("Failed to serialize docker-compose file: {}", e),
    };
    // serialize to file
    std::fs::write(target_file, serialized)?;

    Ok(())
}
