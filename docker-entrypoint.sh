#!/usr/bin/env bash
set -euo pipefail

KEYGEN_DATA="/tmp/keygen_data.txt"
BENCHMARK_DATA="/tmp/benchmark_data.txt"
REPORT_FILE="/output/benchmarks.md"

mkdir -p /output

cd /app/src/zkp/scripts

# ─── Step 1: Key generation ───────────────────────────────────────────────────
# Compiles each circuit to R1CS and writes proving/verifying keys to the
# circuit directories. Extracts R1CS constraint counts into /tmp/keygen_data.txt.
echo ""
echo "==> [1/2] Generating proving/verifying keys..."
./key_gen.sh

# Prover key (P) file sizes — measured immediately after key generation.
pk_offline=$(du -sh /app/src/zkp/new_pcs_poseidon_offline/P 2>/dev/null | awk '{print $1}' || echo "N/A")
pk_online=$(du -sh  /app/src/zkp/new_pcs_poseidon_online/P  2>/dev/null | awk '{print $1}' || echo "N/A")
pk_revoke=$(du -sh  /app/src/zkp/token_revoke/P             2>/dev/null | awk '{print $1}' || echo "N/A")

# ─── Step 2: Benchmark ────────────────────────────────────────────────────────
# Runs prover and verifier 5 times for each circuit; median times are saved
# to /tmp/benchmark_data.txt in addition to being printed to stdout.
echo ""
echo "==> [2/2] Running benchmarks (5 iterations per circuit)..."
./cp_poseidon.sh

# ─── Assemble Markdown report ─────────────────────────────────────────────────
# Read R1CS constraint counts from keygen_data.txt.
r1cs_offline=$(grep "^pcs_poseidon_offline|" "$KEYGEN_DATA" | cut -d'|' -f2 || echo "N/A")
r1cs_online=$(grep  "^pcs_poseidon_online|"  "$KEYGEN_DATA" | cut -d'|' -f2 || echo "N/A")
r1cs_revoke=$(grep  "^token_revoke|"         "$KEYGEN_DATA" | cut -d'|' -f2 || echo "N/A")

# Read timing medians from benchmark_data.txt.
p_offline=$(grep "^prover|pcs_poseidon_offline|"   "$BENCHMARK_DATA" | cut -d'|' -f3 || echo "N/A")
p_revoke=$(grep  "^prover|token_revoke|"           "$BENCHMARK_DATA" | cut -d'|' -f3 || echo "N/A")
p_online=$(grep  "^prover|pcs_poseidon_online|"    "$BENCHMARK_DATA" | cut -d'|' -f3 || echo "N/A")
v_offline=$(grep "^verifier|pcs_poseidon_offline|" "$BENCHMARK_DATA" | cut -d'|' -f3 || echo "N/A")
v_revoke=$(grep  "^verifier|token_revoke|"         "$BENCHMARK_DATA" | cut -d'|' -f3 || echo "N/A")
v_online=$(grep  "^verifier|pcs_poseidon_online|"  "$BENCHMARK_DATA" | cut -d'|' -f3 || echo "N/A")

cat > "$REPORT_FILE" << MDEOF
# 3PaaS-ZKP Benchmark Results

Generated: $(date -u "+%Y-%m-%d %H:%M:%S UTC")

## Timing Benchmarks

| Circuit | Prover Time | Verifier Time |
|---------|------------:|--------------:|
| Derive (t,B) | ${p_offline} | ${v_offline} |
| Non-revocation | ${p_revoke} | ${v_revoke} |
| Derive H | ${p_online} | ${v_online} |

## R1CS Constraints & Prover Key Sizes

| Circuit | R1CS Constraints | Prover Key Size |
|---------|----------------:|----------------:|
| Derive (t,B) | ${r1cs_offline} | ${pk_offline} |
| Non-revocation | ${r1cs_revoke} | ${pk_revoke} |
| Derive H | ${r1cs_online} | ${pk_online} |


MDEOF

echo ""
echo "==> Benchmark report written to $REPORT_FILE"
echo ""
echo "────────────────────────────────────────────────────────────────────────────"
cat "$REPORT_FILE"
echo "────────────────────────────────────────────────────────────────────────────"
echo ""
echo "Done."
