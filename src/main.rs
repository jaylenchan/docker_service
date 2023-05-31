use anyhow::Result;
use docker_service::{
    gen_dockercompose, gen_dockerfile,
    services::{
        Service, {Jenkins, Nginx, Npm},
    },
};
use duct::cmd;

fn main() -> Result<()> {
    gen_files()?;
    run_services()?;

    Ok(())
}

fn gen_files() -> Result<()> {
    let docker_services = vec![
        Service::Nginx(Nginx::new()),
        Service::Jenkins(Jenkins::new()),
        Service::Npm(Npm::new()),
    ];

    gen_dockerfile(&docker_services)?;
    gen_dockercompose(&docker_services)?;

    Ok(())
}

fn run_services() -> Result<()> {
    cmd!("ls").run()?;
    cmd!("cd", "services").run()?;
    cmd!("docker-compose", "up").run()?;

    Ok(())
}
