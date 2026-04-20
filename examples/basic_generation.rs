//! Generate deterministic MIDI and WAV artifacts from a state-space system.
//!
//! Run with:
//!   cargo run --example basic_generation

use nalgebra::DMatrix;
use state_space_music_box::{AudioEngine, StateSpaceSystem};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a 2-state oscillator system
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 0.01, -0.01, 0.999]);
    let b = DMatrix::zeros(2, 0);
    let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
    let d = DMatrix::zeros(1, 0);

    let system = StateSpaceSystem::new(a, b, c, d, Some(0.01))?;
    println!("Controllable: {}", system.is_controllable());
    println!("Observable: {}", system.is_observable());

    // Render 2 seconds of audio
    let mut engine = AudioEngine::new();
    let samples = engine.generate_audio_from_state_space(&system, 2.0, 44_100)?;
    println!(
        "Generated {} audio samples ({:.1}s at 44.1kHz)",
        samples.len(),
        samples.len() as f64 / 44100.0
    );

    // Write WAV file
    let out_path = std::path::PathBuf::from("out/example_basic.wav");
    std::fs::create_dir_all("out")?;
    engine.write_wav_file(&out_path, &samples, 44_100)?;
    println!("Wrote {}", out_path.display());

    Ok(())
}
