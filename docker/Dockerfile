FROM ubuntu:latest
RUN apt-get install libssl-dev
COPY target/release/capsule-server /capsule-server
ENTRYPOINT ["./capsule-server"]
