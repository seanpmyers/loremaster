FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin loremaster

# We do not need the Rust toolchain to run the binary!
FROM ubuntu:20.04 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/loremaster /usr/local/bin
COPY Loremaster.ron /usr/local/bin
COPY certs/ /usr/local/bin/certs
COPY frontend/dist/exported/ /usr/local/bin/frontend/dist/exported
RUN apt-get -y update
RUN apt-get -y install openssl
#RUN apt-get -y install libssl-dev
ENTRYPOINT ["/usr/local/bin/loremaster"]