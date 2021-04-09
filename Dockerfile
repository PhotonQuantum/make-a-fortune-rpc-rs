FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/fortune
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/fortune-rpc-rs /usr/local/bin/fortune-rpc-rs
RUN mkdir /etc/fortune
COPY ./Rocket.toml /etc/fortune/Rocket.toml
ENV ROCKET_CONFIG=/etc/fortune/Rocket.toml
CMD ["fortune-rpc-rs"]
