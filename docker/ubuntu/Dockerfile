FROM rust:1.52 AS builder

WORKDIR build_peat
RUN mkdir git && \
    cd git && \
    git clone https://github.com/broadinstitute/peat.git && \
    git checkout v1.0.0-RC1  && \
    cd peat && \
    cargo build --release --root ../..  && \
    cd ../..  && \
    rm -r git

FROM ubuntu:21.04

RUN apt-get update && \
    apt-get upgrade && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /build_peat/peat /usr/local/bin/peat




