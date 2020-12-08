use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1")]
#[clap(verbatim_doc_comment)]
/// Tendermock
pub struct Args {
    /// Verbode mode
    #[clap(short, long)]
    pub verbose: bool,

    /// JsonRPC port
    #[clap(short, long, default_value = "26657")]
    pub json_port: u16,

    #[clap(short, long, default_value = "50051")]
    pub grpc_port: u16,

    /// Path to json configuration file
    #[clap(short, long)]
    pub config: Option<String>,

    /// Seconds between two blocks, 0 for no growth
    #[clap(short, long, default_value = "20")]
    pub block: u64,
}

/// Parse CLI args, may terminate the program
pub fn get_args() -> Args {
    Args::parse()
}
