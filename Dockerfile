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
    cargo +nightly build --release \
    && mv target/release/filecoindot-template /filecoindot-template

# Release
FROM debian:buster-slim
LABEL description="The docker image of filecoindot template"
COPY --from=builder /filecoindot-template /usr/local/bin/
RUN apt install ca-certificates && \
    useradd -m -u 1000 -U -s /bin/sh -d /pint pint && \
    mkdir -p /pint/.local/share && \
    mkdir /data && \
    chown -R pint:pint /data && \
    ln -s /data /pint/.local/share/filecoindot-template && \
    rm -rf /usr/bin /usr/sbin
USER pint
# 30333 for p2p traffic
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615
VOLUME [ "/data" ]
ENTRYPOINT [ "/usr/local/bin/filecoindot-template" ]
