// Copyright (c) 2022 Milen Dzhumerov

use cheadermap::binary::parse_headermap;

use clap::Parser;
use std::fs;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(clap::AppSettings::PropagateVersion))]
#[clap(global_setting(clap::AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Print(PrintCommand),
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about = "Print the hmap entries", long_about = None)]
struct PrintCommand {
    /// Path to the hmap file.
    #[clap(required = true, parse(from_os_str))]
    path: std::path::PathBuf,
}

impl PrintCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let file_bytes = fs::read(&self.path)?;
        let mut entries = parse_headermap(&file_bytes, true)?;
        entries.sort_by(|lhs, rhs| lhs.key.cmp(rhs.key));
        for entry in entries {
            println!("{} -> {}{}", entry.key, entry.prefix, entry.suffix);
        }
        Ok(())
    }
}

fn execute_command(command: &Commands) -> anyhow::Result<()> {
    match command {
        Commands::Print(print_command) => print_command.execute(),
    }
}

fn main() {
    let cli = Cli::parse();
    let command_result = execute_command(&cli.command);
    match command_result {
        Ok(_) => {}
        Err(err) => {
            println!("{:#?}", err)
        }
    };
}