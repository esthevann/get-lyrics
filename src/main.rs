use clap::{Parser, Subcommand};

use crate::spotify::get_currently_playing_song_info;

mod spotify;
mod genius;

/// Get lyrics by song name using Genius API
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Auth,
    Desktop,
    Manual { artist: String, track: String },
}

fn main() -> miette::Result<()> {
    // let (artist, track) = get_currently_playing_song_info()?;

    // println!("artist: {artist}\ntrack: {track}");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Auth => todo!(),
        Commands::Desktop => todo!(),
        Commands::Manual { artist, track } => {
            todo!()
        }
    }
    Ok(())
}
