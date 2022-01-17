# syntax=docker/dockerfile:experimental
#
# Copyright 2021 ChainSafe Systems
# SPDX-License-Identifier: LGPL-3.0-only
#
# Building layer
FROM paritytech/ci-linux:production as builder
COPY . .
ENV CARGO_TERM_COLOR=always
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,target=target \
    cargo +nightly build --release && \
    mv target/release/filecoindot-template /filecoindot-template

# Release
FROM debian:buster-slim
ENV DEBIAN_FRONTEND=noninteractive
LABEL description="The docker image of filecoindot template"
COPY --from=builder /filecoindot-template /usr/local/bin/
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 1000 -U -s /bin/sh -d /filecoindot filecoindot && \
    mkdir -p /filecoindot/.local/share && \
    mkdir /data && \
    chown -R filecoindot:filecoindot /data && \
    ln -s /data /filecoindot/.local/share/filecoindot-template && \
    rm -rf /usr/bin /usr/sbin

USER filecoindot
# 30333 for p2p traffic
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615
VOLUME [ "/data" ]
ENTRYPOINT [ "/usr/local/bin/filecoindot-template" ]
