FROM --platform=linux/amd64 ubuntu:20.04 AS builder

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y     curl git lsb-release wget gnupg build-essential software-properties-common
RUN wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && ./llvm.sh 12 all
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install -f cargo-fuzz

ADD . /datum
WORKDIR /datum/fuzz
RUN LLVM_SYS_120_PREFIX=/usr/lib/llvm-12 cargo +nightly fuzz build

FROM debian:bookworm
COPY --from=builder /datum/fuzz/target/x86_64-unknown-linux-gnu/release/datum-parser-fuzz /
