use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct HealthOpts {
    #[arg(long, default_value_t = false)]
    pub json: bool,
    #[arg(long, default_value_t = 10000)]
    pub timeout: u64,
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}
