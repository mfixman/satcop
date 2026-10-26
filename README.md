# SATCoP
A theorem prover for first-order logic based on connection tableau and SAT solving.

## Build
Install a Rust toolchain: [rustup](https://rustup.rs) is convenient for this.

```
$ cargo build --release
```

There should now be an executable at `target/release/satcop`.

## Usage
```
$ satcop --help
```

## PicoSAT dependency

SATCoP links against PicoSAT and uses its clausal UNSAT core API. PicoSAT
must be compiled with trace support. Some system libraries omit this feature.
To build a private static library in this checkout:

```sh
curl -fL https://fmv.jku.at/picosat/picosat-965.tar.gz -o /tmp/picosat-965.tar.gz
tar -xzf /tmp/picosat-965.tar.gz -C /tmp
(cd /tmp/picosat-965 && ./configure.sh -O --trace && make -j4 libpicosat.a)
cp /tmp/picosat-965/libpicosat.a .
SATCOP_REVISION="$(git rev-parse HEAD)" RUSTFLAGS="-L native=$PWD" cargo build --release
```

Use the same `RUSTFLAGS` when rebuilding so the private library is found.
The library is ignored by Git. The optional `SATCOP_REVISION` build variable
records the prover revision in collection exports; include a dirty marker if
building uncommitted changes. This checkout currently binds only to PicoSAT;
there is no runtime SAT solver selection.

## Training data export

```sh
target/release/satcop problem.p --threads 1 --seed 0 --time 900 --quiet \
  --collection-output problem.json
```

On a successful refutation, `--collection-output` writes a new JSON file with
schema `satcop.collection.v2`. Existing files are never replaced. The export
contains the entire preprocessed first-order matrix, symbol identities,
formula provenance, conjecture flags, search constraints, actual start clauses,
and the matrix clause IDs originating the ground clauses in PicoSAT's core.
An origin occurs once even if several of its ground instances are in the core.
The core is not guaranteed minimal. No JSON export is written for a timeout,
parse error or GaveUp result. Search panics terminate with a nonzero exit code.

Each thread uses `seed + thread_index`; the export records the winning seed.
The default thread count remains the available CPU count. Collection wrappers
should pass `--threads 1` when parallelizing across problems. `TPTP_DIR` sets
the fallback directory for includes.

The Connections axiom predictor can invoke this executable and convert its
exports into training CSV shards:

```sh
python axiom_selector.py collect PROBLEMS --policy SatCoP \
  --satcop-location ~/TP/satcop --dataset datasets/NativeSATCoP
```

That command runs from the Connections checkout. Its previous Python
implementation is available there as `--policy SATCoPCon`.
