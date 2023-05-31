use anyhow::Result;
use docker_service::{
    ensure_services_dir, gen_files, get_services, set_services_config_folder, set_store_path,
};

fn main() -> Result<()> {
    let store_path = set_store_path("services")?;

    let services_store_info = ensure_services_dir(&get_services(), &store_path)?;

    set_services_config_folder(&get_services(), &services_store_info)?;

    println!("the services store path is: {}", store_path);

    if let Ok(_) = gen_files(&store_path) {
        if let Err(err) = run_services(&store_path) {
            println!("run service err: {}", err);

            stop_services(&store_path)?;
        }
    }

    Ok(())
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
