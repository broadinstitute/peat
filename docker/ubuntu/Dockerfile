FROM rust:1.52 AS builder

WORKDIR build_peat
RUN mkdir git && \
    cd git && \
    git clone https://github.com/broadinstitute/peat.git && \
    cd peat && \
    git checkout v1.0.0  && \
    cargo build --release && \
    mv target/release/peat ../../peat  && \
    cd ../..  && \
    rm -r git

FROM ubuntu:21.04

RUN apt-get -y update && \
    apt-get -y upgrade && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /build_peat/peat /usr/local/bin/peat




