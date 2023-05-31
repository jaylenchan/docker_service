use super::docker::{
    AdvancedBuildStep, BuildStep, Cmd, Dockerfile, DockerfileContent, Dockerservice,
    DockerserviceContent, Expose, Label, Networks, Ports, Run,
};

pub struct Nginx {
    pub docker_file: Dockerfile,
    pub docker_service: Dockerservice,
}

impl Nginx {
    pub fn new() -> Self {
        Nginx {
            docker_file: Dockerfile {
                file_name: "nginx.Dockerfile".to_string(),
                content: DockerfileContent::base("debian:bullseye-slim")
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
            },
            docker_service: Dockerservice {
                service_name: "nginx".into(),
                content: DockerserviceContent {
                    build_: Some(BuildStep::Advanced(AdvancedBuildStep {
                        context: ".".into(),
                        dockerfile: Some("nginx.Dockerfile".into()),
                        ..Default::default()
                    })),
                    ports: Ports::Short(vec!["80:80".into()]),
                    networks: Networks::Simple(vec!["fe_service".into()]),
                    restart: Some("always".into()),
                    ..Default::default()
                },
            },
        }
    }
}
