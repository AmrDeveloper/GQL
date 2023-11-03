use clap::Parser;

fn valid_page_size(input: &str) -> Result<usize, String> {
    let page_size_result = input.parse::<usize>();
    if page_size_result.is_ok() {
        let page_size = page_size_result.ok().unwrap();
        if page_size <= 0 {
            return Err("Page size must be a positive number larger than 0".to_string());
        }
        return Ok(page_size);
    }
    return Err("Invalid page size".to_string());
}

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

    /// Enable Pagination
    #[clap(short, long, action, default_value = "false")]
    pub pagination: bool,

    /// Set page size for Pagination
    #[clap(short, long, action, default_value = "10", value_parser=valid_page_size)]
    pub size_per_page: usize,
}

pub fn parse_arguments() -> Arguments {
    return Arguments::parse();
}
