FROM rust:1.40 as builder
WORKDIR /usr/src/tracker
COPY . .
RUN cargo install --path .

FROM ubuntu:18.04
COPY --from=builder /usr/local/cargo/bin/tracker /tracker
ENTRYPOINT "/tracker"
