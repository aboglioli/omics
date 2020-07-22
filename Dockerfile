# ----------
# Build stage
# ----------
FROM rust:latest AS build

ENV target x86_64-unknown-linux-musl

RUN apt-get update; \
    apt-get install -y musl-tools
RUN rustup target add ${target}

WORKDIR /src
COPY . .

# Build
# RUN cargo build --release --target ${target}
# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/omics*
RUN cargo install --path ./main --target ${target}

# ----------
# Final stage
# ----------
FROM alpine:latest

ENV target x86_64-unknown-linux-musl

COPY --from=build /usr/local/cargo/bin/omics /usr/local/bin/omics

EXPOSE 3000

CMD ["omics"]
