FROM jenkins/jenkins:lts-jdk11
LABEL maintainer JaylenChan <jaylen.work@hotmail.com>
USER root
RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo 'Asia/Shanghai' >/etc/timezone
RUN sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list && sed -i s@/deb.debian.org/@/mirrors.aliyun.com/@g /etc/apt/sources.list
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y --no-install-recommends curl wget vim && apt-get clean && rm -rf /var/lib/apt/lists/*
ENV JAVA_OPTS=-Djenkins.install.runSetupWizard=false
COPY install-plugins.yaml /usr/share/jenkins/ref/install-plugins.yaml
RUN jenkins-plugin-cli --plugin-file /usr/share/jenkins/ref/install-plugins.yaml
ENV ENV CASC_JENKINS_CONFIG=$JENKINS_HOME/casc_configs
ENV mkdir ${CASC_JENKINS_CONFIG}
COPY jenkins.yaml ${CASC_JENKINS_CONFIG}/jenkins.yaml
EXPOSE 8080
