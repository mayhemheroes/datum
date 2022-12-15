FROM ghcr.io/evanrichter/cargo-fuzz as builder

RUN apt update && apt install lsb-release wget software-properties-common gnupg -y && wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && ./llvm.sh 12 all

ADD . /datum
WORKDIR /datum/fuzz
RUN LLVM_SYS_120_PREFIX=/usr/lib/llvm-12 cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /datum/fuzz/target/x86_64-unknown-linux-gnu/release/datum-parser-fuzz /