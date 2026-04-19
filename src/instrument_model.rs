use std::collections::HashMap;

/// Represents an instrument model with parameters
#[derive(Debug, Clone)]
pub struct InstrumentModel {
    /// Name of this model
    pub name: String,
    /// Type of instrument (e.g., 'piano', 'synth', 'drums')
    pub instrument_type: String,
    /// Preset or patch name
    pub preset: Option<String>,
    /// Parameters for this instrument
    pub parameters: HashMap<String, f64>,
}

impl InstrumentModel {
    /// Create a new instrument model
    pub fn new(name: String, instrument_type: String, preset: Option<String>) -> Self {
        InstrumentModel {
            name,
            instrument_type,
            preset,
            parameters: HashMap::new(),
        }
    }

    /// Set a parameter on this instrument
    pub fn set_parameter(&mut self, param: &str, value: f64) {
        self.parameters.insert(param.to_string(), value);
    }

    /// Get a parameter from this instrument
    pub fn get_parameter(&self, param: &str) -> Option<&f64> {
        self.parameters.get(param)
    }

    /// Get all parameters
    pub fn parameters(&self) -> &HashMap<String, f64> {
        &self.parameters
    }

    /// Get the instrument type
    pub fn instrument_type(&self) -> &str {
        &self.instrument_type
    }

    /// Get the preset
    pub fn preset(&self) -> &Option<String> {
        &self.preset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrument_model_creation() {
        let model = InstrumentModel::new(
            "test_piano".to_string(),
            "piano".to_string(),
            Some("grand_piano".to_string())
        );
        assert_eq!(model.name, "test_piano");
        assert_eq!(model.instrument_type, "piano");
        assert_eq!(model.preset, Some("grand_piano".to_string()));
        assert!(model.parameters.is_empty());
    }

    #[test]
    fn test_set_and_get_parameter() {
        let mut model = InstrumentModel::new(
            "test_synth".to_string(),
            "synth".to_string(),
            None
        );
        model.set_parameter("cutoff_frequency", 1000.0);
        model.set_parameter("resonance", 0.5);
        
        assert_eq!(model.get_parameter("cutoff_frequency"), Some(&1000.0));
        assert_eq!(model.get_parameter("resonance"), Some(&0.5));
        assert_eq!(model.get_parameter("nonexistent"), None);
    }

    #[test]
    fn test_parameters() {
        let mut model = InstrumentModel::new(
            "test_drums".to_string(),
            "drums".to_string(),
            Some("rock_kit".to_string())
        );
        model.set_parameter("snare_threshold", 0.7);
        model.set_parameter("kick_decay", 0.3);
        
        let params = model.parameters();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("snare_threshold"), Some(&0.7));
        assert_eq!(params.get("kick_decay"), Some(&0.3));
    }
}