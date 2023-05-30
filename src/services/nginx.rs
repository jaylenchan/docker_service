use dockerfile::{Cmd, Expose, Label, Run};

use crate::services::{Dockerfile, Dockerservice};
pub struct Nginx {
    pub docker_file: Dockerfile,
    pub docker_service: Dockerservice,
}

impl Nginx {
    pub fn new() -> Self {
        Nginx {
            docker_file: Dockerfile {
                content: dockerfile::Dockerfile::base("debian:bullseye-slim")
                    .push(Label::new(
                        "maintainer JaylenChan <jaylen.work@hotmail.com>",
                    ))
                    .push(Run::new(
                        "apt-get update && apt-get install -y \
                    ca-certificates \
                    lua-cjson \
                    lua-iconv \
                    nginx-extras \
                    --no-install-recommends \
                    && rm -rf /var/lib/apt/lists/*",
                    ))
                    .push(Run::new(
                        "ln -sf /dev/stdout /var/log/nginx/access.log \
                    && ln -sf /dev/stderr /var/log/nginx/error.log",
                    ))
                    .push(Expose::new("80 443"))
                    .push(Cmd::new(r#"["nginx", "-g", "daemon off;"]"#))
                    .finish()
                    .to_string(),
                filename: "nginx.Dockerfile".to_string(),
            },
            docker_service: Dockerservice {
                service_name: "npm".to_string(),
                image: "nginx".to_string(),
            },
        }
    }
}
