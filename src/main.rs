use anyhow::Result;
use docker_service::{
    gen_dockercompose, gen_dockerfile,
    services::{Jenkins, Nginx, Npm, Service},
};

fn main() {
    if let Ok(_) = gen_files() {
        run_services();
    }
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

fn run_services() {
    println!("run services!");
}
