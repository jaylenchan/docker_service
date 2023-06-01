use super::docker::{
    AdvancedBuildStep, BuildStep, Cmd, Command, Dockerfile, DockerfileContent, Dockerservice,
    DockerserviceContent, Label, Networks, Run, User,
};

pub struct ElectronBuilder {
    pub docker_file: Dockerfile,
    pub docker_service: Dockerservice,
    pub config_folders: Option<Vec<String>>,
}

impl ElectronBuilder {
    pub fn new() -> Self {
        ElectronBuilder {
            docker_file: Dockerfile {
                file_name: "electron_builder.Dockerfile".to_string(),
                content: DockerfileContent::base("electronuserland/builder:wine")
                    .push(Label::new(
                        "maintainer JaylenChan <jaylen.work@hotmail.com>",
                    ))
                    .push(User::new("root"))
                    .push(Run::new("ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo 'Asia/Shanghai' >/etc/timezone"))
                    .push(Run::new("npm install -g n yarn"))
                    .push(Run::new("n 16.17.1"))
                    .finish()
                    .to_string(),
            },
            docker_service: Dockerservice {
                service_name: "electron_builder".into(),
                content: DockerserviceContent {
                    build_: Some(BuildStep::Advanced(AdvancedBuildStep {
                        context: ".".into(),
                        dockerfile: Some("electron_builder.Dockerfile".into()),
                        ..Default::default()
                    })),
                    image: Some("electron_builder:wizard".into()),
                    container_name: Some("electron_builder".into()),
                    networks: Networks::Simple(vec!["fe_service".into()]),
                    restart: Some("always".into()),
                    tty: true,
                    stdin_open: true,
                    command: Some(Command::Simple("sh".into())),
                    ..Default::default()
                },
            },
            config_folders: None,
        }
    }
}
