use anyhow::Result;
use docker_service::{docker_compose::dockercompose, docker_file::dockerfile};
fn main() -> Result<()> {
    dockerfile()?;
    dockercompose()?;

    Ok(())
}
