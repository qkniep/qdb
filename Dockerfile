FROM rust:latest AS builder
# Compile all cargo dependencies before COPY for caching purposes.
WORKDIR /usr/src/qdb
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --locked && \
    rm -r src
# If there are any changes to the Rust source code, without changing Cargo.toml/lock,
# only the following part should be run; everything above should be in cache.
COPY . .
RUN cargo install --locked --path .

# Run Stage
FROM bitnami/minideb:latest
RUN groupadd -g 1000 qdb && \
    useradd -s /bin/sh -u 1000 -g qdb qdb
WORKDIR /home/qdb/bin/
COPY --from=builder /usr/local/cargo/bin/qdb .
RUN chmod +x qdb && chown qdb:qdb qdb
USER qdb
CMD ["./qdb"]

