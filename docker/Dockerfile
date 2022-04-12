FROM ubuntu:latest
RUN apt-get update && apt-get -y install libssl-dev
COPY target/release/capsule-server /capsule-server
ENTRYPOINT ["./capsule-server"]
