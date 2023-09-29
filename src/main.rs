use clap::Parser;

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();
    let cli = cli::CLI::parse();

    Ok(())
}
