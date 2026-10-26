use crate::block::Id;
use crate::options::Options;
use crate::syntax::{Clause, Matrix, Sort, Source, Term};
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn term(matrix: &Matrix, id: Id<Term>) -> Value {
    let value = matrix.terms[id];
    if value.is_var() {
        return json!({"variable": value.as_var().as_u32()});
    }
    let symbol = value.as_sym();
    let args: Vec<_> = (0..matrix.symbols[symbol].arity)
        .map(|i| term(matrix, matrix.terms[id.offset(i + 1)].as_arg()))
        .collect();
    json!({"symbol": symbol.as_u32(), "args": args})
}

fn clause(matrix: &Matrix, id: Id<Clause>) -> Value {
    let clause = matrix.clauses[id];
    let literals: Vec<_> = clause.literals.into_iter().map(|id| {
        let literal = matrix.literals[id];
        json!({"positive": literal.pol, "atom": term(matrix, literal.atom)})
    }).collect();
    let disequations: Vec<_> = clause
        .disequations
        .into_iter()
        .map(|id| {
            let diseq = matrix.disequations[id];
            json!([term(matrix, diseq.left), term(matrix, diseq.right)])
        })
        .collect();
    let source = match &matrix.info[id].source {
        Source::Equality => json!({"kind": "equality"}),
        Source::Axiom { path, name } => {
            json!({"kind": "input", "path": path.as_ref(), "name": name})
        }
    };
    json!({
        "is_goal": matrix.info[id].is_goal,
        "source": source,
        "variables": clause.vars.as_u32(),
        "literals": literals,
        "disequations": disequations,
    })
}

pub(crate) fn write(
    matrix: &Matrix,
    core: Vec<u32>,
    options: &Options,
    seed: u64,
) -> anyhow::Result<()> {
    let path = match &options.collection_output {
        Some(path) => path,
        None => return Ok(()),
    };
    let symbols: Vec<_> = matrix
        .symbols
        .range()
        .into_iter()
        .map(|id| {
            let symbol = &matrix.symbols[id];
            json!({
                "name": symbol.name.to_string(),
                "arity": symbol.arity,
                "predicate": symbol.sort == Sort::Bool,
            })
        })
        .collect();
    let clauses: Vec<_> = matrix
        .clauses
        .range()
        .into_iter()
        .map(|id| clause(matrix, id))
        .collect();
    let start: Vec<_> = matrix.start.iter().map(|id| id.as_u32()).collect();
    let payload = json!({
        "schema": "satcop.collection.v1",
        "problem_path": options.path,
        "status": if matrix.have_conjecture { "Theorem" } else { "Unsatisfiable" },
        "prover_version": env!("CARGO_PKG_VERSION"),
        "prover_revision": option_env!("SATCOP_REVISION"),
        "seed": seed,
        "threads": options.threads.unwrap_or_else(num_cpus::get),
        "timeout_seconds": options.time,
        "matrix": {
            "symbols": symbols,
            "clauses": clauses,
            "start_clause_ids": start,
            "have_conjecture": matrix.have_conjecture,
            "grounding_constant": matrix.grounding_constant.as_u32(),
        },
        "core_clause_ids": core,
    });
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &payload)?;
    writeln!(writer)?;
    writer.flush()?;
    Ok(())
}
