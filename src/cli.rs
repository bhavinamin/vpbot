use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "vpbot",
    about = "Terminal based algo bot based on volume profile"
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Replay(ReplayArgs),
}

#[derive(Debug, Parser)]
pub struct ReplayArgs {
    #[arg(long)]
    pub file: PathBuf,

    #[arg(long)]
    pub symbol: String,

    #[arg(long)]
    pub tick_size: f64,

    #[arg(long, default_value_t = 0.70)]
    pub value_area: f64,

    #[arg(long)]
    pub stop_ticks: i64,

    #[arg(long)]
    pub target_ticks: i64,

    #[arg(long)]
    pub qty: u32,

    #[arg(long)]
    pub max_events: Option<usize>,
}
