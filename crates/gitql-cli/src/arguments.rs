use clap::Parser;

/// GitQL is a SQL like query language to run on local repositories
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Path for local repositories to run query on
    #[arg(short, long, num_args = 1..)]
    #[structopt(required = true)]
    pub repos: Vec<String>,

    /// Show analysis for front end and engine
    #[clap(short, long, action)]
    pub analysis: bool,
}

pub fn parse_arguments() -> Arguments {
    return Arguments::parse();
}
