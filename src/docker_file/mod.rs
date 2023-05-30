mod jenkins;
mod nginx;
mod npm;

use anyhow::Result;
use jenkins::jenkins_dockerfile;
use nginx::nginx_dockerfile;
use npm::npm_dockerfile;

pub fn gen_dockerfile() -> Result<()> {
    nginx_dockerfile()?;
    npm_dockerfile()?;
    jenkins_dockerfile()?;

    Ok(())
}
