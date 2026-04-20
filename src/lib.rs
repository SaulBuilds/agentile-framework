//! # state-space-music-box
//!
//! A Rust library for turning linear state-space systems into deterministic
//! trajectories, MIDI clips, and WAV audio artifacts.
//!
//! This crate is the mathematical and tooling foundation for the **Agentic DJ**
//! stack. It provides:
//!
//! - **Deterministic generation**: Fixed seed + fixed preset = identical output.
//! - **Multiple surfaces**: Library API, CLI, stdio MCP server, and HTTP API all
//!   call the same backend functions.
//! - **Governance**: Dataset registry, approval tokens, preset snapshots,
//!   append-only audit logs, and durable session state.
//! - **Agent harness**: Constrained planning, mediated execution, and rollback
//!   for AI agents operating over the real tool surface.
//! - **Scheduler**: Immutable unattended jobs with approval-gated scheduling
//!   and exported bundles for Hermes/OpenClaw-style runners.
//! - **Realtime**: OSC adapter store with live preview and transport dispatch.
//!
//! ## Quick Start
//!
//! ```rust
//! use nalgebra::DMatrix;
//! use state_space_music_box::{AudioEngine, StateSpaceSystem};
//!
//! let a = DMatrix::from_row_slice(2, 2, &[1.0, 0.01, -0.01, 0.999]);
//! let b = DMatrix::zeros(2, 0);
//! let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
//! let d = DMatrix::zeros(1, 0);
//!
//! let system = StateSpaceSystem::new(a, b, c, d, Some(0.01)).unwrap();
//! let mut engine = AudioEngine::new();
//! let samples = engine.generate_audio_from_state_space(&system, 1.0, 44_100).unwrap();
//! assert!(samples.len() > 0);
//! ```
//!
//! ## HTTP API
//!
//! Start the HTTP server to expose all tools via REST:
//!
//! ```bash
//! cargo run -- http --port 3001 --api-key my-secret-key
//! ```
//!
//! Then call any tool:
//!
//! ```bash
//! curl -X POST http://localhost:3001/api/tools/list_presets \
//!   -H "Authorization: Bearer my-secret-key" \
//!   -H "Content-Type: application/json" \
//!   -d '{}'
//! ```

pub mod audio_engine;
pub mod cli;
pub mod effect_model;
pub mod generation;
pub mod governance;
pub mod http_server;
pub mod instrument_model;
pub mod mcp;
pub mod midi_model;
pub mod state_machine;
pub mod state_space;
pub mod vst_synthesizer;

// Re-export key types for ease of use
pub use audio_engine::*;
pub use cli::*;
pub use effect_model::*;
pub use generation::*;
pub use governance::*;
pub use http_server::*;
pub use instrument_model::*;
pub use mcp::*;
pub use midi_model::*;
pub use state_machine::*;
pub use state_space::*;
pub use vst_synthesizer::*;

/// Version of the state-space-music-box library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
