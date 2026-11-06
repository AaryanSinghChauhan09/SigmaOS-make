use ::make::{make, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = make::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.run().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-make operational");
    Ok(())
}
