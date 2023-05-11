# Dockerfile for building image for Actix-web app in AWS Lambda
FROM rust:1.69.0 AS builder

# create a new empty shell project
RUN USER=root cargo new --bin tiny-url-server
WORKDIR /tiny-url-server

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this step will cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree
COPY ./adapter ./adapter
COPY ./app ./app
COPY ./driver ./driver
COPY ./kernel ./kernel

# build for release
RUN rm ./target/release/deps/tiny-url-server*
RUN cargo build --release

# final base
FROM rust:1.69.0-slim-buster
COPY --from=builder /tiny-url-server/target/release/tiny-url-server .
CMD ["./tiny-url-server"]

