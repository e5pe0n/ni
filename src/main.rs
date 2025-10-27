use anyhow::{Context, Result};
use clap::Parser;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};
use std::path::PathBuf;
use std::{env, path};

const ENV_VAR_KEY_NI_HOME: &'static str = "NI_HOME";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
}

fn main() -> Result<()> {
    let ni_home_path = match env::var(ENV_VAR_KEY_NI_HOME) {
        Ok(ni_home) => ni_home,
        Err(_) => String::from("~/.ni"),
    };

    let cli = Cli::parse();

    if let Some(name) = &cli.name {
        let path_buf: PathBuf = [&ni_home_path, name].iter().collect();
        let content = fs::read_to_string(&path_buf)
            .with_context(|| format!("failed to read {}.", &path_buf.to_str().unwrap()))?;
        let mut stdout = stdout().lock();
        stdout.write_all(content.as_bytes())?;
        stdout.flush()?;
    }

    Ok(())
}
