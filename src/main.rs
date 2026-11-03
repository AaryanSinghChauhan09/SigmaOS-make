use anyhow::Result;
use clap::Parser;
use make::make;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = make::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = make::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-make operational");
    Ok(())
}
