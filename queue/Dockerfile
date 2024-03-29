# Build: docker build -t api .
FROM rust:alpine as builder

# Set the target feature to -crt-static to build for alpine
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Install musl-dev to build for alpine
RUN apk add --no-cache musl-dev

# Install openssl-dev to build for alpine
RUN apk add openssl-dev

# Install pkgconfig to build for alpine
RUN apk add --no-cache pkgconfig

WORKDIR /

# Copy database files
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

# Copy the Cargo files for dependency resolution
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY ./core/Cargo.toml ./core/Cargo.toml
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./queue/Cargo.toml ./queue/Cargo.toml

#Create dummy files to trick cargo into building the dependencies
RUN mkdir ./core/src && echo "" > ./core/src/lib.rs
RUN mkdir ./queue/src && echo "fn main() {}" > ./queue/src/main.rs

# Build the dependencies
RUN cargo build --release --package marco-polo-rs-queue --bin marco-polo-rs-queue

# Remove dummy files of core and copy the actual files
RUN rm -rf ./core/src
COPY ./core/src ./core/src

# Remove dummy core files compilation artifacts
RUN rm ./target/release/deps/marco_polo_rs_core*
RUN rm ./target/release/deps/libmarco_polo_rs_core*

# Build the core
RUN cargo build --release --package marco-polo-rs-queue --bin marco-polo-rs-queue

# Remove dummy files of queue and copy the actual files
RUN rm -rf ./queue/src
COPY ./queue/src ./queue/src

# Remove dummy queue files compilation artifacts
RUN rm ./target/release/deps/marco_polo_rs_queue*

# Build the queue
RUN cargo build --release --package marco-polo-rs-queue --bin marco-polo-rs-queue

# Copy the binary to a new container
FROM alpine:latest as runtime

# Install openssl-dev
RUN apk add openssl-dev

# Install pkgconfig
RUN apk add --no-cache pkgconfig

# Install musl
RUN apk add --no-cache musl

# Install ffmpeg
RUN apk add --no-cache ffmpeg

# Install arial font
RUN apk add --no-cache ttf-dejavu

# Install yt-dlp
RUN apk add --no-cache python3
RUN apk add --no-cache py3-pip
RUN pip3 install --upgrade pip
RUN pip3 install yt-dlp

# Copy .env file
COPY .env .env

# Copy aws certificate
COPY sa-east-1-bundle.pem sa-east-1-bundle.pem

# Copy Youtube client secret
COPY yt-client-secret.json yt-client-secret.json

# Copy the binary from the builder container
COPY --from=builder /target/release/marco-polo-rs-queue /usr/local/bin/marco-polo-rs-queue

# Run the binary
CMD ["marco-polo-rs-queue"]