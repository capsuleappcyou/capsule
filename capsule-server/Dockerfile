FROM ubuntu:22.04
RUN apt-get update && apt-get -y install libssl-dev && apt-get install -y postgresql-server-dev-all
COPY target/release/capsule-server /capsule-server
COPY config/default.toml /config/default.toml
CMD ["./capsule-server"]
