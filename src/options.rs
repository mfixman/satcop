use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(
    name = "SATCoP",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub(crate) struct Options {
    #[clap(parse(from_os_str), about = "path to input problem")]
    pub(crate) path: PathBuf,

    #[clap(long, about = "Print normal form and exit")]
    pub(crate) clausify: bool,

    #[clap(long, about = "Print search statistcs")]
    pub(crate) statistics: bool,

    #[clap(long, about = "Do not print proof")]
    pub(crate) quiet: bool,

    #[clap(long, about = "enforce time limit (secs)")]
    pub(crate) time: Option<u64>,

    #[clap(
        long,
        parse(from_os_str),
        about = "Write native matrix and SAT-core labels as JSON on success (new file only)"
    )]
    pub(crate) collection_output: Option<PathBuf>,

    #[clap(
        long,
        about = "Number of search threads (default: available CPUs)"
    )]
    pub(crate) threads: Option<usize>,

    #[clap(
        long,
        default_value = "0",
        about = "First search seed; subsequent threads use consecutive seeds"
    )]
    pub(crate) seed: u64,
}

impl Options {
    pub(crate) fn new() -> Self {
        let options = Self::parse();
        assert!(options.threads != Some(0), "--threads must be positive");
        options
    }
}
