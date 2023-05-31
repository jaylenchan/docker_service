use anyhow::Result;
use docker_service::{
    gen_dockercompose, gen_dockerfile,
    services::{
        Service, {Jenkins, Nginx, Npm},
    },
};

fn main() -> Result<()> {
    if let Ok(_) = gen_files() {
        run_services()?;
        // stop_services()?;
    }

    Ok(())
}

fn gen_files() -> Result<()> {
    let docker_services = vec![
        Service::Nginx(Nginx::new()),
        Service::Jenkins(Jenkins::new()),
        // Service::Npm(Npm::new()),
    ];

    gen_dockerfile(&docker_services)?;
    gen_dockercompose(&docker_services)?;

    Ok(())
}

fn run_services() -> Result<()> {
    duct_sh::sh("cd services && docker-compose up").read()?;

    Ok(())
}

#[allow(dead_code)]
fn stop_services() -> Result<()> {
    duct_sh::sh("cd services && docker-compose down").read()?;

    Ok(())
}
