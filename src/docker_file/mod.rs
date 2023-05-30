mod jenkins;
mod nginx;
mod npm;

use anyhow::Result;
use jenkins::jenkins_dockerfile;
use nginx::nginx_dockerfile;
use npm::npm_dockerfile;

use crate::write_to_dir;

pub fn gen_dockerfile() -> Result<()> {
    let dockerfiles: Vec<(String, String)> = vec![
        nginx_dockerfile()?,
        jenkins_dockerfile()?,
        npm_dockerfile()?,
    ];

    dockerfiles.into_iter().for_each(|(dockerfile, filename)| {
        if let Err(_) = write_to_dir("services", &filename, dockerfile) {
            let mut path = String::new();
            path.push_str("services");
            path.push_str(&filename);
            panic!("write_to_dir failed: {}", path);
        };
    });

    Ok(())
}
