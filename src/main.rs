use clap::{Parser, Subcommand};

mod create;
mod help;
mod mount;
mod status;
mod umount;

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
    /// create a new bunker
    Create { name: String, size: u32 },
    /// mount a bunker
    Mount { name: String },
    /// see the status of your bunkers
    Status,
    /// unmount a bunker
    Umount { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name, size } => {
            create::create(name, size);
        }
        Commands::Mount { name } => {
            mount::mount(name);
        }
        Commands::Status => status::status(),
        Commands::Umount { name } => {
            umount::umount(name);
        }
    }
}
