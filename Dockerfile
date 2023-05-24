FROM rust:1.69-slim-buster

WORKDIR /reimbursment

# install pkg-config
RUN apt-get update && apt-get install -y pkg-config libssl-dev
# remove apt cache
RUN rm -rf /var/cache/apk/*

RUN cargo install cargo-watch

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

COPY . .


CMD cargo watch -x run
