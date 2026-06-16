#!/bin/bash

cd ../../rust-rsa-poseidon
RUSTFLAGS="-A warnings" cargo run
cd ../rust-rsa-sha
RUSTFLAGS="-A warnings" cargo run

cd ../non_mem
python3 generate_id.py
python3 non_membership_sub_tree.py
python3 generate_roots.py
python3 non_membership_roots.py

cd ../zkp/prover_inputs
python3 helper.py