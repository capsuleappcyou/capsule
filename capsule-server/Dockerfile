FROM debian:stable
RUN apt-get update && apt-get -y install libssl-dev && apt-get install -y postgresql-server-dev-all
COPY target/release/capsule-server /capsule-server
COPY config/capsule-server.toml /config/capsule-server.toml
CMD ["./capsule-server"]
