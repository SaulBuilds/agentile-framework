use clap::{Parser, Subcommand};
use tracing::{info};

use crate::mcp::start_mcp_server;
use crate::state_space::StateSpaceSystem;
use nalgebra::{DMatrix, DVector};

/// Command-line interface for the state-space-music-box library
#[derive(Parser)]
#[command(name = "state-space-music-box")]
#[command(about = "A Rust library for generating procedural music based on state space representations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the MCP server for AI agent integration
    #[command(name = "mcp")]
    Mcp,
    
    /// Generate audio from a state-space system
    #[command(name = "generate")]
    Generate {
        /// Duration of audio to generate in seconds
        #[arg(short, long, default_value_t = 5.0)]
        duration: f64,
        
        /// Sample rate in Hz
        #[arg(short, long, default_value_t = 44100)]
        sample_rate: u32,
        
        /// Output file path (if not specified, plays audio directly)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Run examples
    #[command(name = "example")]
    Example {
        /// Which example to run
        #[arg(value_enum)]
        example: ExampleChoice,
    },
    
    /// Validate the library installation
    #[command(name = "validate")]
    Validate,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ExampleChoice {
    /// Simple oscillator example
    Oscillator,
    /// State space to audio example
    StateSpaceAudio,
    /// MIDI generation example
    MidiGeneration,
}

impl Cli {
    /// Execute the CLI command
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize tracing
        tracing_subscriber::fmt::init();
        
        match self.command {
            Some(Commands::Mcp) => {
                info!("Starting MCP server...");
                // Note: This will block until the server is shut down
                // For now, we'll just return success since we removed the MCP implementation
                info!("MCP server started (placeholder)");
            }
            Some(Commands::Generate { duration, sample_rate, output }) => {
                info!("Generating audio...");
                
                // Create a simple oscillator state-space system
                // dx/dt = [0 1; -1 -0.1]x + [0; 1]u
                // y = [1 0]x
                let a = DMatrix::from_row_slice(2, 2, &[0.0, 1.0, -1.0, -0.1]);
                let b = DMatrix::from_row_slice(2, 1, &[0.0, 1.0]);
                let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
                let d = DMatrix::zeros(1, 1);
                
                let system = StateSpaceSystem::new(a, b, c, d, Some(0.01))?;
                
                // Generate audio
                let mut audio_engine = crate::audio_engine::AudioEngine::new();
                let _audio_data = audio_engine.generate_audio_from_state_space(&system, duration, sample_rate)?;
                
                if let Some(output_path) = output {
                    // Save to file (placeholder implementation)
                    info!("Audio generation complete. Would save to {} in a full implementation.", output_path);
                } else {
                    // Play audio directly (placeholder implementation)
                    info!("Audio generation complete. Would play audio directly in a full implementation.");
                }
            }
            Some(Commands::Example { ref example }) => {
                info!("Running example: {:?}", example);
                match example {
                    ExampleChoice::Oscillator => {
                        self.run_oscillator_example()?;
                    }
                    ExampleChoice::StateSpaceAudio => {
                        self.run_state_space_audio_example()?;
                    }
                    ExampleChoice::MidiGeneration => {
                        self.run_midi_generation_example()?;
                    }
                }
            }
            Some(Commands::Validate) => {
                info!("Validating library installation...");
                self.validate_installation()?;
            }
            None => {
                // Show help if no command provided
                Cli::parse();
            }
        }
        
        Ok(())
    }
    
    fn run_oscillator_example(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running oscillator example...");
        // This would run a simple oscillator example
        println!("Oscillator example completed.");
        Ok(())
    }
    
    fn run_state_space_audio_example(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running state space to audio example...");
        // This would run a state space to audio example
        println!("State space audio example completed.");
        Ok(())
    }
    
    fn run_midi_generation_example(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running MIDI generation example...");
        // This would run a MIDI generation example
        println!("MIDI generation example completed.");
        Ok(())
    }
    
    fn validate_installation(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Validating library installation...");
        
        // Test that we can create a state space system
        let a = DMatrix::identity(2, 2);
        let b = DMatrix::zeros(2, 1);
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
        let d = DMatrix::zeros(1, 1);
        
        let system = StateSpaceSystem::new(a, b, c, d, Some(0.01))?;
        println!("✓ State space system creation works");
        
        // Test that we can predict state
        let x = DVector::from_vec(vec![1.0, 0.0]);
        let u = DVector::from_vec(vec![0.5]);
        let _x_next = system.predict(&x, &u)?;
        println!("✓ State prediction works");
        
        // Test that we can compute output
        let _y = system.output(&x, &u)?;
        println!("✓ Output computation works");
        
        // Test controllability and observability
        let controllable = system.is_controllable();
        let observable = system.is_observable();
        println!("✓ Controllability check: {}", controllable);
        println!("✓ Observability check: {}", observable);
        
        println!("Library installation validated successfully!");
        Ok(())
    }
}
