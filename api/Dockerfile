# Build: docker build -t api .
FROM rust:alpine as builder

# Set the target feature to -crt-static to build for alpine
ENV RUSTFLAGS="-C target-feature=-crt-static"

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
COPY ./macros/Cargo.toml ./macros/Cargo.toml
COPY ./core/Cargo.toml ./core/Cargo.toml
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./queue/Cargo.toml ./queue/Cargo.toml
COPY ./cli/Cargo.toml ./cli/Cargo.toml

#Create dummy files to trick cargo into building the dependencies
RUN mkdir ./macros/src && echo "" > ./macros/src/lib.rs
RUN mkdir ./core/src && echo "" > ./core/src/lib.rs
RUN mkdir ./api/src && echo "fn main() {}" > ./api/src/main.rs

# Build the dependencies
RUN cargo build --release --package marco-polo-rs-api --bin marco-polo-rs-api

# Remove dummy files of macros and copy the actual files
RUN rm -rf ./macros/src
COPY ./macros/src ./macros/src

# Remove dummy macros files compilation artifacts
RUN rm ./target/release/deps/marco_polo_rs_macros*
RUN rm ./target/release/deps/libmarco_polo_rs_macros*

# Build the macros
RUN cargo build --release --package marco-polo-rs-api --bin marco-polo-rs-api

# Remove dummy files of core and copy the actual files
RUN rm -rf ./core/src
COPY ./core/src ./core/src

# Remove dummy core files compilation artifacts
RUN rm ./target/release/deps/marco_polo_rs_core*
RUN rm ./target/release/deps/libmarco_polo_rs_core*

# Build the core
RUN cargo build --release --package marco-polo-rs-api --bin marco-polo-rs-api

# Remove dummy files of api and copy the actual files
RUN rm -rf ./api/src
COPY ./api/src ./api/src

# Remove dummy api files compilation artifacts
RUN rm ./target/release/deps/marco_polo_rs_api*

# Build the api
RUN cargo build --release --package marco-polo-rs-api --bin marco-polo-rs-api

# Copy the binary to a new container
FROM alpine:latest as runtime

# Install musl to run the binary
RUN apk add --no-cache musl

# Install libgcc
RUN apk add --no-cache libgcc

# Copy the binary from the builder container
COPY --from=builder /target/release/marco-polo-rs-api /usr/local/bin/marco-polo-rs-api

# Copy .env file
COPY .env .env

# Copy aws certificate
COPY sa-east-1-bundle.pem sa-east-1-bundle.pem

# Copy Youtube client secret
COPY yt-client-secret.json yt-client-secret.json

EXPOSE 8080

# Run the binary
CMD ["marco-polo-rs-api"]