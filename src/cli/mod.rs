use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
    #[value(name = "github")]
    GitHub,
}

#[derive(Parser, Debug)]
#[command(name = "lineguard")]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(help = "Files or directories to check")]
    pub files: Vec<String>,

    #[arg(long, help = "Read file paths from stdin")]
    pub stdin: bool,

    #[arg(short, long, help = "Recursively check directories")]
    pub recursive: bool,

    #[arg(
        short,
        long,
        value_enum,
        default_value = "human",
        help = "Output format"
    )]
    pub format: OutputFormat,

    #[arg(short, long, help = "Suppress non-error output")]
    pub quiet: bool,

    #[arg(short, long, help = "Show detailed information")]
    pub verbose: bool,

    #[arg(long, help = "Disable colored output")]
    pub no_color: bool,

    #[arg(short, long, help = "Path to configuration file")]
    pub config: Option<PathBuf>,

    #[arg(long, help = "Ignore files matching pattern")]
    pub ignore: Vec<String>,

    #[arg(
        long,
        value_delimiter = ',',
        help = "File extensions to check (comma-separated)"
    )]
    pub extensions: Option<Vec<String>>,

    #[arg(long, help = "Disable newline ending check")]
    pub no_newline_check: bool,

    #[arg(long, help = "Disable trailing space check")]
    pub no_trailing_space: bool,

    #[arg(long, help = "Automatically fix issues")]
    pub fix: bool,

    #[arg(long, help = "Show what would be fixed without modifying files")]
    pub dry_run: bool,

    #[arg(long, help = "Check files changed since this commit (Git only)")]
    pub from: Option<String>,

    #[arg(
        long,
        help = "Check files changed until this commit (Git only, default: HEAD)"
    )]
    pub to: Option<String>,

    #[arg(long, help = "Skip hidden files (files starting with .)")]
    pub no_hidden: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
