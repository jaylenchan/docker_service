use anyhow::Result;
use dockerfile::Dockerfile;

pub fn npm_dockerfile() -> Result<(String, String)> {
    let dockerfile = Dockerfile::base("verdaccio/verdaccio:nightly-master")
        .finish()
        .to_string();

    let filename = "npm.Dockerfile".to_string();

    Ok((dockerfile, filename))
}
