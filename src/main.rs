use clap::{Args, Parser, Subcommand};
use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write, stdin};
use std::path::{Path, PathBuf};
use std::process;

const ENV_VAR_KEY_NI_HOME: &'static str = "NI_HOME";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Install(InstallArgs),
}

#[derive(Args)]
struct InstallArgs {
    url: String,
}

fn main() {
    let ni_home_path = match env::var(ENV_VAR_KEY_NI_HOME) {
        Ok(ni_home) => ni_home,
        Err(_) => String::from("~/.ni"),
    };

    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        match command {
            Command::Install(args) => {
                let trimmed = &args.url[..&args.url.len() - 4]; // remove ".git"
                let dirname = trimmed
                    .split("/")
                    .last()
                    .expect("failed to extract repo name.");
                let to_path = Path::new(&ni_home_path)
                    .join(dirname)
                    .to_str()
                    .expect("failed to extract dirname.")
                    .to_owned();
                let output = process::Command::new("git")
                    .args(["clone", &args.url, &to_path])
                    .output()
                    .expect("failed to git clone.");
                if !output.status.success() {
                    panic!("failed to git clone.");
                }
            }
        }
    }

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
