use std::collections::HashMap;

/// Represents an effect model with parameters
#[derive(Debug, Clone)]
pub struct EffectModel {
    /// Name of this model
    pub name: String,
    /// Type of effect (e.g., 'reverb', 'delay', 'distortion')
    pub effect_type: String,
    /// Parameters for this effect
    pub parameters: HashMap<String, f64>,
}

impl EffectModel {
    /// Create a new effect model
    pub fn new(name: String, effect_type: String) -> Self {
        EffectModel {
            name,
            effect_type,
            parameters: HashMap::new(),
        }
    }

    /// Set a parameter on this effect
    pub fn set_parameter(&mut self, param: &str, value: f64) {
        self.parameters.insert(param.to_string(), value);
    }

    /// Get a parameter from this effect
    pub fn get_parameter(&self, param: &str) -> Option<&f64> {
        self.parameters.get(param)
    }

    /// Get all parameters
    pub fn parameters(&self) -> &HashMap<String, f64> {
        &self.parameters
    }

    /// Get the effect type
    pub fn effect_type(&self) -> &str {
        &self.effect_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_model_creation() {
        let model = EffectModel::new("test_reverb".to_string(), "reverb".to_string());
        assert_eq!(model.name, "test_reverb");
        assert_eq!(model.effect_type, "reverb");
        assert!(model.parameters.is_empty());
    }

    #[test]
    fn test_set_and_get_parameter() {
        let mut model = EffectModel::new("test_delay".to_string(), "delay".to_string());
        model.set_parameter("delay_time", 0.3);
        model.set_parameter("feedback", 0.5);
        model.set_parameter("mix", 0.7);

        assert_eq!(model.get_parameter("delay_time"), Some(&0.3));
        assert_eq!(model.get_parameter("feedback"), Some(&0.5));
        assert_eq!(model.get_parameter("mix"), Some(&0.7));
        assert_eq!(model.get_parameter("nonexistent"), None);
    }

    #[test]
    fn test_parameters() {
        let mut model = EffectModel::new("test_distortion".to_string(), "distortion".to_string());
        model.set_parameter("drive", 0.8);
        model.set_parameter("tone", 0.6);
        model.set_parameter("level", 0.9);

        let params = model.parameters();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("drive"), Some(&0.8));
        assert_eq!(params.get("tone"), Some(&0.6));
        assert_eq!(params.get("level"), Some(&0.9));
    }
}
