use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rdiff")]
#[command(author, version, about = "A powerful CLI diff tool with web visualization", long_about = None)]
pub struct Args {
    /// First file or directory to compare
    #[arg(value_name = "PATH1")]
    pub path1: String,

    /// Second file or directory to compare
    #[arg(value_name = "PATH2")]
    pub path2: String,

    /// Open diff result in web browser
    #[arg(short = 'w', long = "web")]
    pub web: bool,

    /// Number of context lines in unified diff (default: 3)
    #[arg(short = 'U', long = "unified", default_value = "3")]
    pub unified_lines: usize,

    /// Ignore whitespace changes
    #[arg(long = "ignore-whitespace")]
    pub ignore_whitespace: bool,

    /// Ignore case differences
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Show only file names that differ
    #[arg(short = 'q', long = "brief")]
    pub brief: bool,

    /// Recursively compare directories
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    /// Port for web server (default: random available port)
    #[arg(long = "port")]
    pub port: Option<u16>,

    /// Color output (auto, always, never)
    #[arg(long = "color", default_value = "auto")]
    pub color: String,
}
