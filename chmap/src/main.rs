// Copyright (c) 2022 Milen Dzhumerov

use clap::Parser;

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

#[derive(Copy, Debug, Clone, clap::ArgEnum)]
enum PrintOutputFormat {
    Text,
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about = "Print the hmap entries", long_about = None)]
struct PrintCommand {
    /// The output format.
    #[clap(short, long, arg_enum, default_value_t = PrintOutputFormat::Text)]
    format: PrintOutputFormat,

    /// Path to the hmap file.
    #[clap(required = true, parse(from_os_str))]
    path: std::path::PathBuf,
}

impl PrintCommand {
    fn execute(&self) -> anyhow::Result<()> {
        cheadermap::binary::print_headermap(
            &mut std::io::stdout(),
            &self.path,
            cheadermap::binary::OutputFormat::Text,
        )
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
    let exit_code = match command_result {
        Ok(_) => libc::EXIT_SUCCESS,
        Err(err) => {
            eprintln!("{:#?}", err);
            libc::EXIT_FAILURE
        }
    };

    std::process::exit(exit_code);
}
