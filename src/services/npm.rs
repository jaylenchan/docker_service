// use crate::Service;
use crate::services::{Dockerfile, Dockerservice};
use dockerfile::{Cmd, Expose, Label, Run};
pub struct Npm {
    pub dockerfile: Dockerfile,
    pub service: Dockerservice,
}

impl Npm {
    pub fn new() -> Self {
        Npm {
            dockerfile: Dockerfile {
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
            service: Dockerservice {
                service_name: "nginx".to_string(),
            },
        }
    }
}
