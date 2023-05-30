pub mod services;

use anyhow::Result;
use docker_compose_types::{Compose, Service, Services};
use fs_extra::dir::create_all;
use indexmap::IndexMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn gen_dockerfile(services: &Vec<services::Service>) -> Result<()> {
    services.into_iter().for_each(|service| {
        let dockefile = &service.get_dockerfile();
        if let Err(_) = write_to_dir("services", &dockefile.filename, dockefile.content.clone()) {
            let mut path = String::new();
            path.push_str("services");
            path.push_str(&dockefile.filename);
            panic!("write_to_dir failed: {}", path);
        };
    });

    Ok(())
}

pub fn gen_dockercompose(services: &Vec<services::Service>) -> Result<()> {
    let mut services = IndexMap::new();

    let service = (
        "web".to_string(),
        Some(Service {
            image: Some("nginx:latest".to_string()),
            ..Default::default()
        }),
    );
    services.insert(service.0, service.1);

    let serialized = serde_yaml::to_string(&Compose {
        version: Some("3.8".to_string()),
        services: { Services(services) },
        ..Default::default()
    })?;

    write_to_dir("services", "docker-compose.yml", serialized)?;

    Ok(())
}

pub fn write_to_dir(dir: &str, filename: &str, content: String) -> Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        create_all(path, true)?;
    }

    let create_file_result = File::create(&path.join(filename));

    match create_file_result {
        Ok(mut file) => {
            if let Err(_) = write!(&mut file, "{}", content) {
                panic!("Failed to write file: {}", &path.to_string_lossy());
            }
        }
        Err(_) => {
            panic!("Failed to create file: {}", &path.to_string_lossy());
        }
    }

    Ok(())
}
