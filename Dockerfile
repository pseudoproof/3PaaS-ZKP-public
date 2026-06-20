# syntax=docker/dockerfile:1
#
# 3PaaS-ZKP Benchmark Image
#
# Platform: linux/amd64 (x86_64)
#
# docker build -t 3paas-zkp .
# docker run --rm 3paas-zkp

###############################################################################
# Stage 1 — Builder
# Compiles sig-pop (circ + run_zk) and the RSA blind-signature input generator.
###############################################################################
FROM --platform=linux/amd64 ubuntu:22.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        pkg-config \
        libssl-dev \
        libgmp-dev \
        libmpfr-dev \
        m4 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust 1.89.0 (pinned to match the tested toolchain)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain 1.89.0 --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /build

COPY src src

# Build sig-pop — produces the circ compiler and the run_zk prover/verifier
RUN cd src/sig-pop && \
    RUSTFLAGS="-A warnings" cargo build --release \
        --example circ \
        --example run_zk \
        --features r1cs,smt,zok,spartan

# Build the RSA blind-signature input generator (binary: blind-rsa-signatures)
RUN cd src/rust-rsa-poseidon && \
    RUSTFLAGS="-A warnings" cargo build --release


###############################################################################
# Stage 2 — Runtime
###############################################################################
FROM --platform=linux/amd64 ubuntu:22.04 AS runtime

ENV DEBIAN_FRONTEND=noninteractive

# cvc4  — SMT solver invoked by circ during key generation (--action setup)
# python3 — non-membership Merkle-tree scripts
# lib*   — shared libraries required by the compiled Rust binaries
RUN apt-get update && apt-get install -y --no-install-recommends \
        cvc4 \
        python3 \
        libssl3 \
        libgmp10 \
        libmpfr6 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ── Compiled binaries ─────────────────────────────────────────────────────────
# key_gen.sh and cp_poseidon.sh use the relative path
#   ../../sig-pop/target/release/examples/{circ,run_zk}
# when run from src/zkp/new_pcs_poseidon_{offline,online}/ or src/zkp/token_revoke/.
# Placing the binaries at this exact path keeps the scripts unmodified.
RUN mkdir -p src/sig-pop/target/release/examples

COPY --from=builder \
    /build/src/sig-pop/target/release/examples/circ \
    src/sig-pop/target/release/examples/circ

COPY --from=builder \
    /build/src/sig-pop/target/release/examples/run_zk \
    src/sig-pop/target/release/examples/run_zk

# RSA input generator (optional — pre-bundled inputs are used by default)
COPY --from=builder \
    /build/src/rust-rsa-poseidon/target/release/blind-rsa-signatures \
    src/rust-rsa-poseidon/blind-rsa-signatures

# ── ZoKrates source files needed by circ at key-generation time ───────────────
# The .zok circuits import from ../../sig-pop/zok_src/ (relative to each
# circuit file), so the full source tree must be present alongside the binary.
COPY src/sig-pop/zok_src src/sig-pop/zok_src

# ZoKrates standard library — resolved at runtime via ZSHARP_STDLIB_PATH
COPY src/sig-pop/third_party/ZoKrates/zokrates_stdlib/stdlib \
     src/sig-pop/third_party/ZoKrates/zokrates_stdlib/stdlib

# ── ZKP circuits, scripts, and pre-bundled prover inputs ─────────────────────
# The repo ships with ready-to-use test inputs in src/zkp/prover_inputs/.
# Regenerating them requires running rust-rsa-poseidon and the non_mem
# Python scripts (see README — the root-tree step takes significant time).
COPY src/zkp src/zkp
RUN chmod +x /app/src/zkp/scripts/*.sh

# ── Non-membership Merkle-tree Python scripts ─────────────────────────────────
COPY src/non_mem src/non_mem

ENV ZSHARP_STDLIB_PATH=/app/src/sig-pop/third_party/ZoKrates/zokrates_stdlib/stdlib

COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Benchmark report is written here; mount a host directory to persist it.
#   docker run -v "$(pwd)/output:/output" 3paas-zkp
RUN mkdir -p /output
VOLUME ["/output"]

ENTRYPOINT ["/app/docker-entrypoint.sh"]
