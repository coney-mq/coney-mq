FROM rust:alpine AS builder

USER root

RUN apk add --no-cache git openssh musl-dev openssl-dev
RUN rustup toolchain uninstall stable &&\
    rustup toolchain install stable &&\
    rustup update stable &&\
    rustup component add clippy &&\
    export USER=root &&\
    mkdir /src

ADD ./libs /src/libs
ADD ./services /src/services
ADD ./Cargo.toml /src/Cargo.toml
ADD ./Cargo.lock /src/Cargo.lock

WORKDIR /src
RUN cargo build --release

FROM alpine:latest

# RUN echo '@testing http://nl.alpinelinux.org/alpine/edge/testing' >> /etc/apk/repositories
# RUN apk add datamash@testing

RUN mkdir -p /app/sbin
COPY --from=builder /src/target/release/coney-mq-server /app/sbin/coney-mq-server