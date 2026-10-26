use crate::options::Options;
use crate::syntax::{Matrix, Sort, Source};
use anyhow::Context;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn quote(value: &str) -> String {
    let mut text = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => text.push_str("\\\""),
            '\\' => text.push_str("\\\\"),
            '\0'..='\u{1f}' => text.push_str(&format!("\\u{:04x}", ch as u32)),
            _ => text.push(ch),
        }
    }
    text.push('"');
    text
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
    let problem = options.path.to_str().context("non-UTF-8 problem path")?;
    let symbols = matrix
        .symbols
        .range()
        .into_iter()
        .map(|id| {
            let sym = &matrix.symbols[id];
            format!(
                "[{},{},{}]",
                quote(&sym.name.to_string()),
                sym.arity,
                sym.sort == Sort::Bool
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let sources = matrix
        .clauses
        .range()
        .into_iter()
        .map(|id| match &matrix.info[id].source {
            Source::Equality => "null".into(),
            Source::Axiom { path, name } => {
                format!("[{},{}]", quote(path), quote(name))
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    let terms: Vec<_> = matrix
        .terms
        .range()
        .into_iter()
        .map(|id| matrix.terms[id])
        .collect();
    let literals: Vec<_> = matrix
        .literals
        .range()
        .into_iter()
        .map(|id| {
            let lit = matrix.literals[id];
            [lit.pol as u32, lit.atom.as_u32()]
        })
        .collect();
    let disequations: Vec<_> = matrix
        .disequations
        .range()
        .into_iter()
        .map(|id| {
            let diseq = matrix.disequations[id];
            [diseq.left.as_u32(), diseq.right.as_u32()]
        })
        .collect();
    let clauses: Vec<_> = matrix
        .clauses
        .range()
        .into_iter()
        .map(|id| {
            let clause = matrix.clauses[id];
            [
                clause.vars.as_u32(),
                clause.literals.start.as_u32(),
                clause.literals.stop.as_u32(),
                clause.disequations.start.as_u32(),
                clause.disequations.stop.as_u32(),
                matrix.info[id].is_goal as u32,
            ]
        })
        .collect();
    let start: Vec<_> = matrix.start.iter().map(|id| id.as_u32()).collect();
    let revision = option_env!("SATCOP_REVISION")
        .map(quote)
        .unwrap_or_else(|| "null".into());
    let timeout = options
        .time
        .map(|time| time.to_string())
        .unwrap_or_else(|| "null".into());
    let status = if matrix.have_conjecture {
        "Theorem"
    } else {
        "Unsatisfiable"
    };

    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    let mut writer = BufWriter::new(file);
    write!(
        writer,
        r#"{{"schema":"satcop.collection.v2","problem_path":{},"status":"{}","prover_version":"{}","prover_revision":{},"seed":{},"threads":{},"timeout_seconds":{},"matrix":{{"#,
        quote(problem),
        status,
        env!("CARGO_PKG_VERSION"),
        revision,
        seed,
        options.threads.unwrap_or_else(num_cpus::get),
        timeout
    )?;
    write!(
        writer,
        r#""symbols":[{}],"sources":[{}],"terms":{:?},"literals":{:?},"disequations":{:?},"clauses":{:?},"#,
        symbols, sources, terms, literals, disequations, clauses
    )?;
    writeln!(
        writer,
        r#""start_clause_ids":{:?},"have_conjecture":{},"grounding_constant":{}}},"core_clause_ids":{:?}}}"#,
        start,
        matrix.have_conjecture,
        matrix.grounding_constant.as_u32(),
        core
    )?;
    writer.flush()?;
    Ok(())
}
