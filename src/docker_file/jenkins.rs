use anyhow::Result;
use dockerfile::{Copy, Dockerfile, Env, Expose, Label, Run, User};
use std::{fs::File, io::Write};

pub fn jenkins_dockerfile() -> Result<()> {
    let dockerfile =  Dockerfile::base("jenkins/jenkins:lts-jdk11")
    .push(Label::new("maintainer JaylenChan <jaylen.work@hotmail.com>"))
    .push(User::new("root"))
    .push(Run::new("ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo 'Asia/Shanghai' >/etc/timezone"))
    .push(Run::new("sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list && sed -i s@/deb.debian.org/@/mirrors.aliyun.com/@g /etc/apt/sources.list"))
    .push(Run::new("DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y --no-install-recommends curl wget vim && apt-get clean && rm -rf /var/lib/apt/lists/*"))
    .push(Env::new("JAVA_OPTS=-Djenkins.install.runSetupWizard=false"))
    .push(Copy::new("install-plugins.yaml /usr/share/jenkins/ref/install-plugins.yaml"))
    .push(Run::new("jenkins-plugin-cli --plugin-file /usr/share/jenkins/ref/install-plugins.yaml"))
    .push(Env::new("ENV CASC_JENKINS_CONFIG=$JENKINS_HOME/casc_configs"))
    .push(Env::new("mkdir ${CASC_JENKINS_CONFIG}"))
    .push(Copy::new("jenkins.yaml ${CASC_JENKINS_CONFIG}/jenkins.yaml"))
    .push(Expose::new("8080"))
    .finish();

    let mut file = File::create("jenkins.Dockerfile")?;

    write!(&mut file, "{}", dockerfile.to_string())?;

    Ok(())
}
