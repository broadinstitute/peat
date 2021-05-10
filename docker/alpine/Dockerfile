FROM rust:1.52-alpine3.13 AS builder

WORKDIR build_peat
RUN apk update && \
    apk upgrade && \
    apk add git && \
    mkdir git && \
    cd git && \
    git clone https://github.com/broadinstitute/peat.git && \
    cd peat && \
    git checkout v1.0.0  && \
    cargo build --release && \
    mv target/release/peat ../../peat  && \
    cd ../..  && \
    rm -r git

FROM alpine:3.13

RUN apk update && \
    apk upgrade

COPY --from=builder /build_peat/peat /usr/local/bin/peat




