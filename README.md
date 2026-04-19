# state-space-music-box

A Rust library for generating procedural music based on state space representations.

## Overview

state-space-music-box translates mathematical state space models into audible musical experiences. This library enables:

- Real-time generative music from dynamical systems
- Educational tools for understanding complex systems through sound
- Artistic applications that bridge mathematics and music
- Research platform for sonification of multi-dimensional data

## Features

- **State Space to Audio Mapping**: Convert linear time-invariant systems to musical parameters
- **Real-time Audio Generation**: Low-latency audio output suitable for interactive applications
- **Multiple Synthesis Approaches**: FM, additive, and granular synthesis options
- **Visualization Tools**: Debug and visualize state trajectories alongside audio output
- **Cross-platform**: Works on Linux, macOS, and Windows
- **No_std Support**: Core functionality available in embedded contexts

## Getting Started

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
state-space-music-box = "0.1.0"
```

### Basic Usage

```rust
use state_space_music_box::{StateSpaceSystem, Synthesizer, AudioOutput};

// Define a simple harmonic oscillator
let system = StateSpaceSystem::new(
    ndarray::array![[0.0, 1.0], [-1.0, 0.0]], // A matrix
    ndarray::array![[0.0], [1.0]],             // B matrix
    ndarray::array![[1.0, 0.0]],               // C matrix
    ndarray::array![[0.0]],                    // D matrix
);

// Create a synthesizer that maps state variables to audio parameters
let mut synth = Synthesizer::new(system)
    .with_fm_synthesis()
    .with_state_to_frequency_mapping([0, 1])    // Map state vars 0&1 to frequency
    .with_state_to_amplitude_mapping([0]);      // Map state var 0 to amplitude

// Generate audio
let mut audio_output = AudioOutput::new(44100); // 44.1kHz sample rate
let audio_buffer = synth.generate_audio(2.0);   // Generate 2 seconds of audio
audio_output.play_buffer(&audio_buffer);
```

## Project Status

This project is currently in the **GREENFIELD** phase. Initial development is focused on:

1. Core state space representation and manipulation
2. Basic audio synthesis backends
3. Mapping strategies from state variables to audio parameters
4. Real-time audio output implementation
5. Comprehensive test suite for mathematical correctness

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/state-space-music-box.git
cd state-space-music-box

# Install dependencies
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_oscillator
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Citation

If you use this library in academic work, please cite:

```bibtex
@manual{state-space-music-box,
  title = {state-space-music-box: A Library for Procedural Music from State Space Representations},
  author = {Your Name},
  year = {2026},
  url = {https://github.com/yourusername/state-space-music-box}
}
```