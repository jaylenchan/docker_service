pub mod services;

use anyhow::Result;
use fs_extra::dir::create_all;
use indexmap::IndexMap;
use os_info::{get as get_os_info, Type};
use services::{docker::*, Jenkins, Nginx, Service};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn gen_files(store_path: &str) -> Result<()> {
    let docker_services = vec![
        Service::Nginx(Nginx::new()),
        Service::Jenkins(Jenkins::new()),
        // Service::Npm(Npm::new()),
    ];

    gen_dockerfile(&docker_services, store_path)?;
    gen_dockercompose(&docker_services, store_path)?;

    Ok(())
}

pub fn set_store_path(root_folder: &str) -> Result<String> {
    let info = get_os_info();

    match info.os_type() {
        Type::Macos | Type::Ubuntu => match dirs::home_dir() {
            Some(path_buf) => {
                let store_path = path_buf
                    .join(root_folder)
                    .into_os_string()
                    .into_string()
                    .unwrap();

                Ok(store_path)
            }
            None => {
                panic!("can't set store path!")
            }
        },
        _ => unreachable!(),
    }
}

pub fn gen_dockerfile(services: &Vec<Service>, store_path: &str) -> Result<()> {
    services.into_iter().for_each(|service| {
        let dockerfile = &service.get_docker_file();
        if let Err(_) = write_to_dir(
            store_path,
            &dockerfile.file_name,
            dockerfile.content.clone(),
        ) {
            panic!(
                "write_to_dir failed: {}",
                Path::new(store_path)
                    .join(&dockerfile.file_name)
                    .to_string_lossy()
            );
        };
    });

    Ok(())
}

pub fn gen_dockercompose(services: &Vec<Service>, store_path: &str) -> Result<()> {
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

    write_to_dir(store_path, "docker-compose.yml", serialized)?;

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
