use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello transformers!!");

    Ok(())
}

#[cfg(test)]
mod transformer_test;
mod query;
