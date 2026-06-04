# Build Stage
FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

# Add source code
ADD . /src
WORKDIR /src

# Build fuzzers
RUN cd fuzz && cargo fuzz build

# Package Stage
FROM ubuntu:22.04
COPY --from=builder /src/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/
