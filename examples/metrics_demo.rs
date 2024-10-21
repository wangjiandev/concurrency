use anyhow::Result;
use concurrency::metrics::Metrics;

fn main() -> Result<()> {
    let mut metrics = Metrics::default();
    metrics.inc("test");
    metrics.dec("test");

    for _ in 0..10 {
        metrics.inc("test2");
    }
    println!("{:?}", metrics.snapshot());
    Ok(())
}
