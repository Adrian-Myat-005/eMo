use clap::{Parser as ClapParser, Subcommand};
use emo_compiler::{compile, run, format_file};

#[derive(ClapParser)]
#[command(name = "eMo Compiler")]
#[command(version = "4.0")]
#[command(about = "The Unified eMo Compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a .emo project or file into a native binary
    Build {
        file: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Run a script directly (using interpreter)
    Run {
        file: String,
    },
    /// Format an eMo file
    Fmt {
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { file, output } => {
            compile(file, output.clone());
        }
                Commands::Run { file } => {
                    run(file);
                }
                Commands::Fmt { file } => {
                    format_file(file);
                }
            }
        }
        