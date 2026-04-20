//! Demonstrate the full session workflow: create, preview, play, evaluate.
//!
//! Run with:
//!   cargo run --example session_workflow

use state_space_music_box::generation::{demo_preset, save_preset};
use state_space_music_box::governance::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let preset_dir = tmp.path().join("presets");
    let runtime_dir = tmp.path().join("runtime");
    let session_store = default_session_store_path(&runtime_dir);

    // Save a preset
    let mut preset = demo_preset();
    preset.name = "example-demo".to_string();
    save_preset(&preset, &preset_dir)?;
    println!("Saved preset: {}", preset.name);

    // Create a session
    let session = create_session(
        &session_store,
        &preset_dir,
        NewSessionRequest {
            display_name: "Example Session".to_string(),
            preset_name: "example-demo".to_string(),
            seed: 42,
            actor_id: "example-user".to_string(),
        },
    )?;
    println!(
        "Created session: {} ({})",
        session.display_name, session.session_id
    );

    // Render a preview
    let preview = render_session_preview(
        &session_store,
        &preset_dir,
        &runtime_dir,
        &session.session_id,
        "example-user",
    )?;
    println!(
        "Rendered preview: {} ({} notes)",
        preview.preview.preview_id, preview.preview.midi.note_count
    );
    println!("  MIDI: {}", preview.preview.midi.path.display());
    println!("  WAV:  {}", preview.preview.wav.path.display());

    // Start playing
    let playing = apply_transport_command(
        &session_store,
        &session.session_id,
        SessionTransportRequest {
            actor_id: "example-user".to_string(),
            command: SessionTransportCommand::Play,
            run_label: Some("example-run".to_string()),
        },
    )?;
    println!("Session status: {:?}", playing.status);

    // Stop
    let stopped = apply_transport_command(
        &session_store,
        &session.session_id,
        SessionTransportRequest {
            actor_id: "example-user".to_string(),
            command: SessionTransportCommand::Stop,
            run_label: None,
        },
    )?;
    println!("Session status: {:?}", stopped.status);

    println!(
        "\nDone! All session state persisted to: {}",
        runtime_dir.display()
    );
    Ok(())
}
