ENV target x86_64-unknown-linux-musl

# ----
# Build stage
# ----
FROM rust:latest AS build

RUN apt-get update; \
    apt-get install -y musl-tools
RUN rustup target add ${target}

WORKDIR /src
COPY . .

# Lint
RUN cargo fmt -- --check
RUN cargo clippy -- -Dwarnings

# Test
RUN cargo test

# Build
RUN cargo build --release --target ${target}
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/omics*
# RUN cargo install --path ./main

# ---
# Final stage
# ---
FROM alpine:latest

COPY --from=build /src/target/${target}/release/omics /usr/local/bin/omics

EXPOSE 3000

CMD ["omics"]
