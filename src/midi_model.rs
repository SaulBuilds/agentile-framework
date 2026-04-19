

/// Represents a single MIDI note
#[derive(Debug, Clone)]
pub struct MidiNote {
    /// MIDI note number (0-127)
    pub note: u8,
    /// Velocity (0-127)
    pub velocity: u8,
    /// Start time in beats
    pub start_time: f64,
    /// Duration in beats
    pub duration: f64,
}

impl MidiNote {
    /// Create a new MIDI note
    pub fn new(note: u8, velocity: u8, start_time: f64, duration: f64) -> Self {
        MidiNote {
            note,
            velocity,
            start_time,
            duration,
        }
    }
}

/// Represents a MIDI model that can generate MIDI data
#[derive(Debug, Clone)]
pub struct MidiModel {
    /// Name of this model
    pub name: String,
    /// MIDI channel (0-15)
    pub channel: u8,
    /// Default velocity (0-127)
    pub default_velocity: u8,
    /// List of notes in this model
    pub notes: Vec<MidiNote>,
}

impl MidiModel {
    /// Create a new MIDI model
    pub fn new(name: String, channel: u8, default_velocity: u8) -> Self {
        MidiModel {
            name,
            channel: channel.min(15), // Ensure channel is 0-15
            default_velocity: default_velocity.min(127), // Ensure velocity is 0-127
            notes: Vec::new(),
        }
    }

    /// Add a note to this model
    pub fn add_note(&mut self, note: MidiNote) {
        self.notes.push(note);
        // Sort notes by start time for proper MIDI generation
        self.notes.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
    }

    /// Get all notes in this model
    pub fn notes(&self) -> &Vec<MidiNote> {
        &self.notes
    }

    /// Get the MIDI channel
    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// Get the default velocity
    pub fn default_velocity(&self) -> u8 {
        self.default_velocity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_model_creation() {
        let model = MidiModel::new("test_model".to_string(), 0, 64);
        assert_eq!(model.name, "test_model");
        assert_eq!(model.channel, 0);
        assert_eq!(model.default_velocity, 64);
        assert!(model.notes.is_empty());
    }

    #[test]
    fn test_add_note() {
        let mut model = MidiModel::new("test_model".to_string(), 0, 64);
        model.add_note(MidiNote::new(60, 100, 0.0, 1.0)); // Middle C
        model.add_note(MidiNote::new(62, 100, 1.0, 1.0)); // D
        
        assert_eq!(model.notes.len(), 2);
        assert_eq!(model.notes[0].note, 60);
        assert_eq!(model.notes[1].note, 62);
    }

    #[test]
    fn test_note_sorting() {
        let mut model = MidiModel::new("test_model".to_string(), 0, 64);
        model.add_note(MidiNote::new(60, 100, 2.0, 1.0)); // Should be second
        model.add_note(MidiNote::new(62, 100, 0.0, 1.0)); // Should be first
        
        assert_eq!(model.notes.len(), 2);
        assert_eq!(model.notes[0].note, 62); // Earlier start time
        assert_eq!(model.notes[1].note, 60); // Later start time
    }
}