use anyhow::Result;
use docker_service::{
    gen_dockercompose, gen_dockerfile,
    services::{
        Service, {Jenkins, Nginx},
    },
};
use os_info::{get as get_os_info, Type};

fn main() -> Result<()> {
    let store_path = set_store_path("services")?;

    println!("the services store path is: {}", store_path);

    if let Ok(_) = gen_files(&store_path) {
        if let Err(err) = run_services(&store_path) {
            println!("run service err: {}", err);

            stop_services(&store_path)?;
        }
    }

    Ok(())
}

fn gen_files(store_path: &str) -> Result<()> {
    let docker_services = vec![
        Service::Nginx(Nginx::new()),
        Service::Jenkins(Jenkins::new()),
        // Service::Npm(Npm::new()),
    ];

    gen_dockerfile(&docker_services, store_path)?;
    gen_dockercompose(&docker_services, store_path)?;

    Ok(())
}

fn set_store_path(root_folder: &str) -> Result<String> {
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

#[allow(dead_code)]
fn run_services(store_path: &str) -> Result<()> {
    docker_compose_up(store_path)?;
    Ok(())
}

#[allow(dead_code)]
fn stop_services(store_path: &str) -> Result<()> {
    docker_compose_down(store_path)?;
    force_remove_all_container()?;

    Ok(())
}

#[allow(dead_code)]
fn docker_compose_up(store_path: &str) -> Result<()> {
    duct_sh::sh_dangerous(format!(
        "docker-compose -f {}/docker-compose.yml up -d",
        store_path
    ))
    .read()?;

    Ok(())
}

#[allow(dead_code)]
fn docker_compose_down(store_path: &str) -> Result<()> {
    duct_sh::sh_dangerous(format!(
        "docker-compose -f {}/docker-compose.yml down -d",
        store_path
    ))
    .read()?;
    Ok(())
}

#[allow(dead_code)]
fn force_remove_all_container() -> Result<()> {
    duct_sh::sh_dangerous("docker container rm $(docker ps -aq) -f").read()?;

    Ok(())
}

#[allow(dead_code)]
fn force_remove_all_images() -> Result<()> {
    duct_sh::sh_dangerous("docker image rm $(docker images -aq) -f").read()?;

    Ok(())
}
