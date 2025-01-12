use std::{fs::File, io::Read, path::PathBuf};

pub mod builtins;
pub mod conf;
pub mod macros;
pub mod opcode;
pub mod state;
pub(crate) mod types;
pub mod vm;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
#[command(name = "lattice")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        file: PathBuf,
    },
    Run {
        file: PathBuf,
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    run_cli()
}

fn run_cli() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { file: _ } => todo!(),
        Commands::Run { file, debug } => {
            let mut file = File::open(file)?;

            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            let mut vm = vm::VM::new();

            vm.set_debug_mode(debug);
            vm.set_program(buf);
            vm.check_magic();
            vm.check_version();
            vm.load_consts();
            vm.jump_to_code();
            vm.run();
        }
    }

    Ok(())
}
