use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// Use non-default firefox profile
    #[arg(short, long)]
    pub profile: Option<String>,
    /// Use non-default firefox profile dir
    #[arg(long)]
    pub profile_dir: Option<PathBuf>,
    /// Filter the cookies by domain
    /// (matches `<DOMAIN>` and `.<DOMAIN>`)
    #[arg(short, long)]
    pub domain: Option<String>,
    /// Cookie output format
    #[arg(value_enum, long, short, default_value_t)]
    pub output_format: OutputFormat,
}

#[derive(Default, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Default javascript format
    #[default]
    Javascript,
    /// Netscape format, compatible with curl & wget.
    Netscape,
    /// Json format
    Json,
}
