pub mod services;

use anyhow::Result;
use fs_extra::dir::create_all;
use indexmap::IndexMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use services::{docker::*, Service};

pub fn gen_dockerfile(services: &Vec<Service>) -> Result<()> {
    services.into_iter().for_each(|service| {
        let dockefile = &service.get_docker_file();
        if let Err(_) = write_to_dir("services", &dockefile.file_name, dockefile.content.clone()) {
            let mut path = String::new();
            path.push_str("services");
            path.push_str(&dockefile.file_name);
            panic!("write_to_dir failed: {}", path);
        };
    });

    Ok(())
}

pub fn gen_dockercompose(services: &Vec<Service>) -> Result<()> {
    let mut docker_services = IndexMap::new();
    services.into_iter().for_each(|service| {
        let docker_service = service.get_docker_service();

        docker_services.insert(docker_service.service_name, Some(docker_service.content));
    });

    let serialized = serde_yaml::to_string(&Compose {
        version: Some("3.8".to_string()),
        services: { Services(docker_services) },
        networks: {
            let mut map = IndexMap::new();
            map.insert(
                "fe_service".to_string(),
                MapOrEmpty::Map(NetworkSettings {
                    name: Some("fe_service".into()),
                    driver: Some("bridge".into()),
                    ..Default::default()
                }),
            );
            ComposeNetworks(map)
        },
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
