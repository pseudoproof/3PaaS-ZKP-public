#!/bin/bash
set -eo pipefail

# Saves one line per circuit to /tmp/keygen_data.txt:
#   <circuit_label>|<final_r1cs_constraint_count>
KEYGEN_DATA="/tmp/keygen_data.txt"
> "$KEYGEN_DATA"

# Relative path to the circ binary — resolved from each circuit directory after cd.
CIRC="./../../sig-pop/target/release/examples/circ"

# Run key generation for one circuit, display output in real-time, and
# extract the post-optimisation R1CS constraint count.
run_setup() {
    local label="$1"
    local circuit_file="$2"
    local tmpf
    tmpf=$(mktemp)

    # tee lets the output stream to the terminal while we also capture it.
    $CIRC --ram true "$circuit_file" r1cs --action setup --proof-impl mirage 2>&1 \
        | tee "$tmpf"

    local r1cs_size
    r1cs_size=$(grep "Final R1cs size:" "$tmpf" | awk '{print $4}' | tail -1 || true)
    echo "${label}|${r1cs_size:-N/A}" >> "$KEYGEN_DATA"
    rm -f "$tmpf"
}

# Key Gen: generate prover and verifier keys for all three circuits.
cd ../new_pcs_poseidon_offline
run_setup "pcs_poseidon_offline" "./pcs_poseidon_offline.zok"

cd ../new_pcs_poseidon_online
run_setup "pcs_poseidon_online" "./pcs_poseidon_online.zok"

cd ../token_revoke
run_setup "token_revoke" "./token_revoke.zok"

cd ..
