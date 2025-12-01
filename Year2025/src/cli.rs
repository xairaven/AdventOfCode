use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The day to run
    #[arg(short, long)]
    pub day: u8,
    /// Part 1 or 2?
    #[arg(short, long)]
    pub part: u8,
}
