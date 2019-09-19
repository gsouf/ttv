use std::path::PathBuf;

use structopt::StructOpt;

use crate::split::{ProportionSplit, RowSplit};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ttv",
    about = "Flexibly create test, train and validation sets"
)]
pub struct Opt {
    #[structopt(
        parse(from_occurrences),
        short = "v",
        help = "Set the level of verbosity"
    )]
    pub verbose: u64,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(
        name = "split",
        about = "Split dataset into two or more files for test/train/validation sets"
    )]
    Split(Split),
}

#[derive(Debug, StructOpt)]
pub struct Split {
    #[structopt(
        short = "r",
        long = "rows",
        required_unless = "prop_splits",
        conflicts_with = "prop_splits",
        help = "Specify splits by number of rows",
        use_delimiter = true
    )]
    pub row_splits: Vec<RowSplit>,

    #[structopt(
        short = "p",
        long = "prop",
        required_unless = "row_splits",
        conflicts_with = "row_splits",
        help = "Specify splits by proportion of rows",
        use_delimiter = true
    )]
    pub prop_splits: Vec<ProportionSplit>,

    #[structopt(
        short = "c",
        long = "chunk-size",
        help = "Maximum number of rows per output chunk"
    )]
    pub chunk_size: Option<u64>,

    #[structopt(
        short = "t",
        long = "total-rows",
        help = "Number of rows in input file. Used for progress when using proportion splits"
    )]
    pub total_rows: Option<u64>,

    #[structopt(short = "s", long = "seed", help = "RNG seed, for reproducibility")]
    pub seed: Option<u64>,

    #[structopt(
        parse(from_os_str),
        help = "Data to split, optionally gzip compressed. If '-', read from stdin"
    )]
    pub input: PathBuf,

    #[structopt(
        short = "o",
        long = "output-prefix",
        parse(from_os_str),
        required_if("input", "-"),
        help = "Output filename prefix. Only used if reading from stdin"
    )]
    pub output_prefix: Option<PathBuf>,

    #[structopt(
        short = "u",
        long = "uncompressed-input",
        help = "Treat input as uncompressed"
    )]
    pub uncompressed: bool,

    #[structopt(long = "uncompressed-output", help = "Don't compress output files")]
    pub uncompressed_output: bool,
}
