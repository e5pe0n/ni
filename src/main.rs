use clap::Parser;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write, stdin};
use std::path::PathBuf;

const ENV_VAR_KEY_NI_HOME: &'static str = "NI_HOME";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let ni_home_path = match env::var(ENV_VAR_KEY_NI_HOME) {
        Ok(ni_home) => ni_home,
        Err(_) => String::from("~/.ni"),
    };

    let cli = Cli::parse();

    if let Some(name) = &cli.name {
        let path_buf: PathBuf = [&ni_home_path, name].iter().collect();
        let buf_reader = BufReader::new(stdin().lock());
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path_buf)
            .expect("failed to open file.");
        for line in buf_reader.lines() {
            if let Ok(line) = line {
                _ = file.write((line + "\n").as_bytes());
            }
        }
    }
}
