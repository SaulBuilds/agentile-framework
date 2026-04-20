use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

/// Supported plugin bundle formats understood by the library.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginBundleFormat {
    Vst3,
}

/// Filesystem metadata for a validated plugin reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginMetadata {
    pub file_name: String,
    pub is_directory: bool,
    pub byte_size: u64,
}

/// A validated reference to an external VST bundle.
///
/// This type is intentionally limited to filesystem validation and metadata.
/// It does not simulate hosting, parameter automation, or MIDI playback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VstSynthesizer {
    pub name: String,
    pub plugin_path: PathBuf,
    pub bundle_format: PluginBundleFormat,
    pub metadata: PluginMetadata,
}

impl VstSynthesizer {
    /// Create a validated VST plugin reference from a real bundle path.
    pub fn new(name: String, plugin_path: String) -> Result<Self> {
        let plugin_path = PathBuf::from(plugin_path);
        validate_plugin_path(&plugin_path)?;
        let metadata = metadata_for_path(&plugin_path)?;

        Ok(Self {
            name,
            plugin_path,
            bundle_format: PluginBundleFormat::Vst3,
            metadata,
        })
    }

    /// Re-read filesystem metadata for the referenced bundle.
    pub fn refresh(&mut self) -> Result<()> {
        validate_plugin_path(&self.plugin_path)?;
        self.metadata = metadata_for_path(&self.plugin_path)?;
        Ok(())
    }

    pub fn plugin_path(&self) -> &Path {
        &self.plugin_path
    }
}

fn validate_plugin_path(plugin_path: &Path) -> Result<()> {
    ensure!(
        plugin_path.exists(),
        "VST3 bundle does not exist: {}",
        plugin_path.display()
    );

    let file_name = plugin_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let has_vst3_suffix = plugin_path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.eq_ignore_ascii_case("vst3"))
        .unwrap_or(false)
        || file_name.ends_with(".vst3");

    ensure!(
        has_vst3_suffix,
        "unsupported plugin path '{}': expected a .vst3 bundle",
        plugin_path.display()
    );

    Ok(())
}

fn metadata_for_path(plugin_path: &Path) -> Result<PluginMetadata> {
    let metadata = fs::metadata(plugin_path)?;
    let file_name = plugin_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string();

    Ok(PluginMetadata {
        file_name,
        is_directory: metadata.is_dir(),
        byte_size: metadata.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_vst_reference_creation() {
        let dir = tempdir().unwrap();
        let plugin_dir = dir.path().join("demo.vst3");
        fs::create_dir(&plugin_dir).unwrap();

        let plugin =
            VstSynthesizer::new("demo".to_string(), plugin_dir.display().to_string()).unwrap();

        assert_eq!(plugin.name, "demo");
        assert_eq!(plugin.bundle_format, PluginBundleFormat::Vst3);
        assert!(plugin.metadata.is_directory);
    }

    #[test]
    fn test_vst_reference_missing_path_fails() {
        let error = VstSynthesizer::new("demo".to_string(), "/tmp/does-not-exist.vst3".to_string())
            .unwrap_err();

        assert!(error.to_string().contains("does not exist"));
    }

    #[test]
    fn test_vst_reference_invalid_extension_fails() {
        let dir = tempdir().unwrap();
        let invalid_path = dir.path().join("demo.txt");
        fs::write(&invalid_path, b"not a plugin").unwrap();

        let error = VstSynthesizer::new("demo".to_string(), invalid_path.display().to_string())
            .unwrap_err();

        assert!(error.to_string().contains(".vst3"));
    }
}
