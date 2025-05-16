FROM archlinux:latest

WORKDIR /app

COPY target/release/poti ./

VOLUME ["/config"]
VOLUME ["/backup"]

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

EXPOSE 3000