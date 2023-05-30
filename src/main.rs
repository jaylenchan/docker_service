use anyhow::Result;
use docker_service::{docker_compose::gen_dockercompose, docker_file::gen_dockerfile};

fn main() -> Result<()> {
    gen_dockerfile()?;
    gen_dockercompose()?;

    Ok(())
}
