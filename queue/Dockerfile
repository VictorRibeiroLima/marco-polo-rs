# Build: docker build -t api .
FROM rust:alpine as builder

# Install musl-dev to build for alpine
RUN apk add --no-cache musl-dev

# Install openssl-dev to build for alpine
RUN apk add --no-cache openssl-dev

# Install pkgconfig to build for alpine
RUN apk add --no-cache pkgconfig

WORKDIR /

# Copy database files
COPY sqlx-data.json sqlx-data.json
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

# Copy .env file
COPY .env .env

# Install musl to run the binary
RUN apk add --no-cache musl

# Install yt-dlp
RUN apk add --no-cache ffmpeg

# Copy the binary from the builder container
COPY --from=builder /target/release/marco-polo-rs-queue /usr/local/bin/marco-polo-rs-queue

EXPOSE 8080

# Run the binary
CMD ["marco-polo-rs-queue"]