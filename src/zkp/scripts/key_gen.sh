#!/bin/bash

# Key Gen: generate prover and verifier key:
cd ../new_pcs_poseidon_offline
./../../sig-pop/target/release/examples/circ --ram true ./pcs_poseidon_offline.zok r1cs --action setup --proof-impl mirage

cd ../new_pcs_poseidon_online
./../../sig-pop/target/release/examples/circ --ram true ./pcs_poseidon_online.zok r1cs --action setup --proof-impl mirage

cd ../token_revoke
./../../sig-pop/target/release/examples/circ --ram true ./token_revoke.zok r1cs --action setup --proof-impl mirage

cd ..