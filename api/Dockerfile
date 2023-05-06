# Build: docker build -t api .
FROM rust:alpine as builder

# Install musl-dev to build for alpine
RUN apk add --no-cache musl-dev

# Install openssl-dev to build for alpine
RUN apk add --no-cache openssl-dev

# Install pkgconfig to build for alpine
RUN apk add --no-cache pkgconfig

WORKDIR /

COPY . .

RUN rm .env

# Build dependencies
RUN cargo build --release --bin marco-polo-rs-api

# Copy the binary to a new container
FROM alpine:latest as runtime

# Copy .env file
COPY .env .env

# Install musl to run the binary
RUN apk add --no-cache musl

# Install yt-dlp
RUN apk add --no-cache ffmpeg
RUN apk add --no-cache python3
RUN apk add --no-cache py3-pip
RUN pip3 install --upgrade pip
RUN pip3 install yt-dlp

# Copy the binary from the builder container
COPY --from=builder /target/release/marco-polo-rs-api /usr/local/bin/marco-polo-rs-api

EXPOSE 8080

# Run the binary
CMD ["marco-polo-rs-api"]