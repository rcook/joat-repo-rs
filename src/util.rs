use anyhow::Result;
use std::io::{stdin, stdout, Write};

pub fn prompt(message: &str) -> Result<String> {
    print!("{}: ", message);
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    Ok(line.trim().to_lowercase())
}
