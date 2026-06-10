use clap::{Parser, Subcommand};

mod create;
mod help;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "bunkers")]
#[command(version = &VERSION)]
#[command(about = "A CLI tool to manage LUKS-encrypted disk images", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { name: String, size: u32 },
    Mount { name: String },
    Umount { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name, size } => {
            create::create(name, size);
        }
        Commands::Mount { name } => {
            println!("Called mount with {}", name);
        }
        Commands::Umount { name } => {
            println!("Called umount with {}", name);
        }
    }
}
