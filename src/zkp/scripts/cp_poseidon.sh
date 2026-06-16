#!/bin/bash

CIRC_PATH="./../../sig-pop/target/release/examples/circ"
RUN_ZK_PATH="./../../sig-pop/target/release/examples/run_zk"

# SHA_PROVER_INPUT="../prover_inputs/input_sha.txt"
POSEIDON_PROVER_INPUT="../prover_inputs/input_poseidon.txt"
TOKEN_REVOCATION_INPUT="../prover_inputs/token_revocation.txt"

cd ../new_pcs_poseidon_offline


results=()
for i in $(seq 1 5); do
   output=$($RUN_ZK_PATH --action prove --compute pcs-poseidon-preprocess --proof-impl mirage --aux-input $POSEIDON_PROVER_INPUT)
   time_line=$(echo "$output" | awk 'NR==2')
   exact_time_w_unit=$(echo "$time_line" | awk '{print $5}')
   results+=("$exact_time_w_unit")
done

sorted=($(printf "%s\n" "${results[@]}" | sort -g))
# echo "${sorted[@]}"
median=${sorted[2]}
# echo "(Offline) prover time to blind (median over 5 iterations): $median"
echo "Time to gen proof for Derive (t,B): $median"

cd ../token_revoke
results=()
for i in $(seq 1 5); do
   output=$($RUN_ZK_PATH --action prove --compute non-member --proof-impl mirage --aux-input $TOKEN_REVOCATION_INPUT)
   time_line=$(echo "$output" | awk 'NR==2')
   exact_time_w_unit=$(echo "$time_line" | awk '{print $5}')
   results+=("$exact_time_w_unit")
done

sorted=($(printf "%s\n" "${results[@]}" | sort -g))
# echo "${sorted[@]}"
median=${sorted[2]}
# echo "(Offline) prover time to revoke (median over 5 iterations): $median"
echo "Time to gen proof for non-revocation: $median"

cd ../new_pcs_poseidon_online

results=()
# Note : the compute is the same as in the offline but the inputs are differents (cannot be preprocessed)
for i in $(seq 1 5); do
   output=$($RUN_ZK_PATH --action prove --compute pcs-poseidon-preprocess --proof-impl mirage --aux-input $POSEIDON_PROVER_INPUT)
   time_line=$(echo "$output" | awk 'NR==2')
   exact_time_w_unit=$(echo "$time_line" | awk '{print $5}')
   results+=("$exact_time_w_unit")
done

sorted=($(printf "%s\n" "${results[@]}" | sort -g))
median=${sorted[2]}
echo "Time to gen proof for Derive H: $median"

# verifier time 
cd ../new_pcs_poseidon_offline
# echo "(proof composition) Blind a token (SHA-256) and check for revocation - Online and Offline phases:"
# echo "Mutli-thread results:"

results=()
for i in $(seq 1 5); do
   output=$($RUN_ZK_PATH --action verify --compute pcs-poseidon-preprocess --proof-impl mirage --aux-input $POSEIDON_PROVER_INPUT)
   time_line=$(echo "$output" | awk 'NR==2')
   exact_time_w_unit=$(echo "$time_line" | awk '{print $5}')
   results+=("$exact_time_w_unit")
done

sorted=($(printf "%s\n" "${results[@]}" | sort -g))
median_veri1=${sorted[2]}


cd ../token_revoke
results=()
for i in $(seq 1 5); do
   output=$($RUN_ZK_PATH --action verify --compute non-member --proof-impl mirage --aux-input $TOKEN_REVOCATION_INPUT)
   time_line=$(echo "$output" | awk 'NR==2')
   exact_time_w_unit=$(echo "$time_line" | awk '{print $5}')
   results+=("$exact_time_w_unit")
done

sorted=($(printf "%s\n" "${results[@]}" | sort -g))
median_veri2=${sorted[2]}

cd ../new_pcs_poseidon_online

results=()
# Note : the compute is the same as in the offline but the inputs are differents
for i in $(seq 1 5); do
   output=$($RUN_ZK_PATH --action verify --compute pcs-poseidon-preprocess --proof-impl mirage --aux-input $POSEIDON_PROVER_INPUT)
   time_line=$(echo "$output" | awk 'NR==2')
   exact_time_w_unit=$(echo "$time_line" | awk '{print $5}')
   results+=("$exact_time_w_unit")
done

sorted=($(printf "%s\n" "${results[@]}" | sort -g))
median_veri3=${sorted[2]}

echo "Total verifier time: $median_veri1" +" $median_veri2" + "$median_veri3"
# echo "$median_veri1 + $median_veri2 + $median_veri3" | bc