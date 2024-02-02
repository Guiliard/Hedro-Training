FROM rust:latest AS build

WORKDIR /build
COPY . .
RUN apt update && apt install cmake -y && cargo build --release

FROM rust:slim-bookworm

WORKDIR /app
COPY --from=build /build/.env /build/target/release/rmq-consumer /app/