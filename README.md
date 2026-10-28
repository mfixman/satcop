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

## External collection

`satcop problem.p --collect --seed 0` prints essential matrix debug data and
matrix clause IDs for ground clauses in the UNSAT core. An external wrapper
can capture stdout and translate it into training data. PicoSAT must have
trace support enabled. See `packages/axiom_prediction/README.md` in the
Connections repository for the wrapper and build instructions.

## Clause weights

`satcop problem.p --weights weights.txt` reads one positive weight per matrix
clause, in the clause order printed by `--collect`. Each extension step then
orders its alternatives by a weighted random permutation, proportional to the
clause weights, instead of shuffling them uniformly.
