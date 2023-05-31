use super::docker::{
    AdvancedBuildStep, BuildStep, Dockerfile, DockerfileContent, Dockerservice,
    DockerserviceContent, Env, Expose, Label, Networks, Ports, Run, User,
};

pub struct Jenkins {
    pub docker_file: Dockerfile,
    pub docker_service: Dockerservice,
}

impl Jenkins {
    pub fn new() -> Self {
        Jenkins {
            docker_file: Dockerfile {
                file_name: "jenkins.Dockerfile".to_string(),
                content:  DockerfileContent::base("jenkins/jenkins:lts-jdk11")
                .push(Label::new("maintainer JaylenChan <jaylen.work@hotmail.com>"))
                .push(User::new("root"))
                .push(Run::new("ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo 'Asia/Shanghai' >/etc/timezone"))
                .push(Run::new("sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list && sed -i s@/deb.debian.org/@/mirrors.aliyun.com/@g /etc/apt/sources.list"))
                .push(Run::new("DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y --no-install-recommends curl wget vim && apt-get clean && rm -rf /var/lib/apt/lists/*"))
                .push(Env::new("JAVA_OPTS=-Djenkins.install.runSetupWizard=false"))
                // .push(Copy::new("install-plugins.yaml /usr/share/jenkins/ref/install-plugins.yaml"))
                // .push(Run::new("jenkins-plugin-cli --plugin-file /usr/share/jenkins/ref/install-plugins.yaml"))
                // .push(Env::new("ENV CASC_JENKINS_CONFIG=$JENKINS_HOME/casc_configs"))
                // .push(Env::new("mkdir ${CASC_JENKINS_CONFIG}"))
                // .push(Copy::new("jenkins.yaml ${CASC_JENKINS_CONFIG}/jenkins.yaml"))
                .push(Expose::new("8080"))
                .finish()
                .to_string(),
            },
            docker_service: Dockerservice {
                service_name: "jenkins".into(),
                content: DockerserviceContent {
                    build_: Some(BuildStep::Advanced(AdvancedBuildStep {
                        context: ".".into(),
                        dockerfile: Some("jenkins.Dockerfile".into()),
                        ..Default::default()
                    })),
                    image:  Some("jenkins:wizard".into()),
                    container_name: Some("jenkins".into()),
                    ports: Ports::Short(vec!["8080:8080".into()]),
                    networks: Networks::Simple(vec!["fe_service".into()]),
                    restart: Some("always".into()),
                    ..Default::default()
                }
            },
        }
    }
}
