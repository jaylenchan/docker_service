use anyhow::Result;
use docker_service::{gen_dockercompose, gen_dockerfile, services::get_services};

fn main() {
    if let Ok(_) = gen_files() {
        run_services();
    }
}

fn gen_files() -> Result<()> {
    let docker_services = get_services();
    gen_dockerfile(&docker_services)?;
    gen_dockercompose(&docker_services)?;

    Ok(())
}

fn run_services() {
    println!("run services!");
}
