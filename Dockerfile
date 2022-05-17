# Workaround for QEmu bug when building for 32bit platforms on a 64bit host
FROM --platform=$BUILDPLATFORM rust:latest as vendor
ARG BUILDPLATFORM
ARG TARGETPLATFORM
RUN echo "Running on: $BUILDPLATFORM / Building for $TARGETPLATFORM"
WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src src
RUN mkdir .cargo && cargo vendor > .cargo/config.toml

FROM rust:latest as builder
WORKDIR /app

COPY --from=vendor /app/.cargo .cargo
COPY --from=vendor /app/vendor vendor
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src src
RUN cargo +nightly build --release

FROM debian:buster-slim
WORKDIR /
ENV RUST_BACKTRACE=true
COPY --from=builder /app/target/release/hubitat_exporter /hubitat_exporter

EXPOSE 8000
ENTRYPOINT ["/hubitat_exporter"]