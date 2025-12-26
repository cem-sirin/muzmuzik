mod note;
mod widgets;
mod audio;
mod instruments;
mod piano;
mod ui;

use cpal::traits::{DeviceTrait, StreamTrait};
use audio::AudioEngine;
use audio::get_default_output_device;
use clap::Parser;

const WAVEFORM_BUFFER_MS: usize = 200;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, help = "Custom keyboard mapping (24 ASCII characters)")]
    keys: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(mapping) = cli.keys {
        if let Err(e) = piano::set_keyboard_mapping(mapping) {
            eprintln!("Error: {}", e);
            eprintln!("Usage: {} --keys \"<24-character-string>\"", std::env::args().next().unwrap_or("music".to_string()));
            eprintln!("Example: music --keys \"abcdefghijklmnopqrstuvwxyz\"");
            std::process::exit(1);
        }
    }

    let (device, config) = get_default_output_device()?;
    let sample_rate = config.sample_rate;

    let engine = AudioEngine::new(sample_rate, WAVEFORM_BUFFER_MS);
    let audio_callback = engine.build_audio_callback();
    let stream = device.build_output_stream(
        &config,
        audio_callback.into_callback(),
        |err| eprintln!("audio error: {}", err),
        None,
    )?;

    stream.play()?;

    ui::run_tui(&engine, sample_rate)?;

    Ok(())
}
