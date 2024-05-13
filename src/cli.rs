use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, required = false, default_value = "%I %T:0 %D")]
    /// Format string with space-separated identifiers
    pub format: String,

    #[arg(short, long, required = false, value_parser = ["standard", "metric", "imperial"], default_value = "standard")]
    /// Unit specifier
    pub units: String,

    #[arg(short, long)]
    /// Alternative path to credential file
    pub credentials: Option<String>,
}
