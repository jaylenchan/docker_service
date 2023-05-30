FROM debian:bullseye-slim
LABEL maintainer JaylenChan <jaylen.work@hotmail.com>
RUN apt-get update && apt-get install -y ca-certificates lua-cjson lua-iconv nginx-extras --no-install-recommends && rm -rf /var/lib/apt/lists/*
RUN ln -sf /dev/stdout /var/log/nginx/access.log && ln -sf /dev/stderr /var/log/nginx/error.log
EXPOSE 80 443
CMD ["nginx", "-g", "daemon off;"]
