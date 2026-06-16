#!/usr/bin/env bash
set -euo pipefail

cd /app/src/zkp/scripts

# ─── Step 1: Key generation ───────────────────────────────────────────────────
# Compiles each circuit to R1CS and writes proving/verifying keys to the
# circuit directories (zkp/new_pcs_poseidon_{offline,online}/, zkp/token_revoke/).
# This is the slow step — expect several minutes on first run.
echo ""
echo "==> [1/2] Generating proving/verifying keys..."
./key_gen.sh

# ─── Step 2: Benchmark ────────────────────────────────────────────────────────
# Runs prover and verifier 5 times for each of the three circuits and reports
# the median wall-clock time.
echo ""
echo "==> [2/2] Running benchmarks (5 iterations per circuit)..."
./cp_poseidon.sh

echo ""
echo "Done."
