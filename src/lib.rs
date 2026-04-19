pub mod state_machine;
pub mod state_space;
pub mod midi_model;
pub mod instrument_model;
pub mod effect_model;
pub mod vst_synthesizer;
pub mod audio_engine;
pub mod cli;
pub mod mcp;

// Re-export key types for ease of use
pub use state_machine::*;
pub use state_space::*;
pub use midi_model::*;
pub use instrument_model::*;
pub use effect_model::*;
pub use vst_synthesizer::*;
pub use audio_engine::*;
pub use cli::*;
pub use mcp::*;

/// Version of the state-space-music-box library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");