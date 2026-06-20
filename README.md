# 3PaaS-ZKP

This repository contains the zero-knowledge proof (ZKP) circuits and benchmarking scripts from *3PaaS: Privacy-Preserving Post-Compromise Security as a Service*.

## Content

```
.
├── src/
│   ├── non_mem/              # Builds the revocation Merkle tree and non-membership witnesses
│   ├── rust-rsa-poseidon/    # Generates RSA blind-signature prover inputs
│   ├── sig-pop/              # CirC ZKP compiler infrastructure
│   └── zkp/                  # ZoKrates circuits and scripts
│       └── prover_inputs/    # Pre-generated prover inputs (ready to use)
├── Dockerfile                # Dockerfile for running the benchmark
├── docker-entrypoint.sh      # Container entrypoint: key generation + benchmark
└── README.md
```

## Dependencies

The circuits are compiled using [CirC](https://github.com/circify/circ) with the ZoKrates frontend and the Mirage proof backend. To reproduce the results, you need to either install the dependencies directly (see Native section below), or use the provided Dockerfile to run everything in a container.

## Benchmark

The benchmark measures the prover and verifier wall-clock time for three ZKP circuits:

|    Circuit     |                                    What it proves                                |
|----------------|----------------------------------------------------------------------------------|
| Derive(t,B)    | The token 't' and blinded value 'B' are well formed                              |
| Derive H       | The hash H is correctly derived                                                  |
| Non-revocation | The user's identity is absent from the revocation Merkle tree                    |

Each circuit is run 5 times and the median time is reported.

## Reproducing the Results

### Using Docker (recommended)

```
docker build -t 3paas-zkp .
docker run --rm -v "$(pwd)/output:/output" 3paas-zkp
```

The container will generate proving/verifying keys for all three circuits, run the benchmark automatically, and write a Markdown report to `/output/benchmarks.md` (mounted to `./output/benchmarks.md` on the host). The report includes R1CS constraint counts, prover key sizes, and median prover/verifier times for all three circuits. The report is also printed to the terminal at the end of the run.

### Native

- [Rust](https://rustup.rs/) / cargo 1.89.0
- cvc4 
- Python 3
- libgmp, libmpfr, libssl

From the `src/zkp/scripts` directory:

**1. Build the CirC compiler**:

```
chmod +x ./circ_setup.sh
source ./circ_setup.sh
```

You must `source` (not execute) this script so the `ZSHARP_STDLIB_PATH` environment variable persists in your shell.

**2. Generate proving and verifying keys:**

```
chmod +x ./key_gen.sh
./key_gen.sh
```

**3. Run the benchmark:**

```
chmod +x ./cp_poseidon.sh
./cp_poseidon.sh
```

The repository already includes pre-bundled prover inputs in `zkp/prover_inputs/`. Regenerating them is optional and can be slow due to Merkle tree construction:

```
# chmod +x ./input_gen.sh
# ./input_gen.sh
```
