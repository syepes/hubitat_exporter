FROM ubuntu:latest

RUN apt-get update && \
  apt-get -y upgrade&& \
  apt-get install -y curl && \
  rm -rf /var/lib/apt/lists/*

ADD target/*/release/hubitat_exporter /hubitat_exporter

WORKDIR /

CMD ["hubitat_exporter"]