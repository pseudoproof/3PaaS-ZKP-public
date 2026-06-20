#!/bin/bash

RUN_ZK_PATH="./../../sig-pop/target/release/examples/run_zk"

POSEIDON_PROVER_INPUT="../prover_inputs/input_poseidon.txt"
TOKEN_REVOCATION_INPUT="../prover_inputs/token_revocation.txt"

# Saves one line per circuit+action to /tmp/benchmark_data.txt:
#   <action>|<circuit_label>|<median_time>
BENCHMARK_DATA="/tmp/benchmark_data.txt"
> "$BENCHMARK_DATA"

# Run run_zk 5 times and return the median wall-clock time.
# Usage: get_median <action> <compute> <input_file>
get_median() {
    local action="$1"
    local compute="$2"
    local input="$3"
    local results=()

    for i in $(seq 1 5); do
        local output
        output=$($RUN_ZK_PATH --action "$action" --compute "$compute" \
                     --proof-impl mirage --aux-input "$input")
        local t
        t=$(echo "$output" | awk 'NR==2 {print $5}')
        results+=("$t")
    done

    local sorted=($(printf "%s\n" "${results[@]}" | sort -g))
    echo "${sorted[2]}"
}

# ── Prover benchmarks ──────────────────────────────────────────────────────────
cd ../new_pcs_poseidon_offline
median=$(get_median prove pcs-poseidon-preprocess "$POSEIDON_PROVER_INPUT")
echo "Time to gen proof for Derive (t,B): $median"
echo "prover|pcs_poseidon_offline|${median}" >> "$BENCHMARK_DATA"

cd ../token_revoke
median=$(get_median prove non-member "$TOKEN_REVOCATION_INPUT")
echo "Time to gen proof for non-revocation: $median"
echo "prover|token_revoke|${median}" >> "$BENCHMARK_DATA"

cd ../new_pcs_poseidon_online
median=$(get_median prove pcs-poseidon-preprocess "$POSEIDON_PROVER_INPUT")
echo "Time to gen proof for Derive H: $median"
echo "prover|pcs_poseidon_online|${median}" >> "$BENCHMARK_DATA"

# ── Verifier benchmarks ────────────────────────────────────────────────────────
cd ../new_pcs_poseidon_offline
median=$(get_median verify pcs-poseidon-preprocess "$POSEIDON_PROVER_INPUT")
echo "verifier|pcs_poseidon_offline|${median}" >> "$BENCHMARK_DATA"

cd ../token_revoke
median=$(get_median verify non-member "$TOKEN_REVOCATION_INPUT")
echo "verifier|token_revoke|${median}" >> "$BENCHMARK_DATA"

cd ../new_pcs_poseidon_online
median=$(get_median verify pcs-poseidon-preprocess "$POSEIDON_PROVER_INPUT")
echo "verifier|pcs_poseidon_online|${median}" >> "$BENCHMARK_DATA"
