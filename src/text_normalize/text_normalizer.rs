use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rules {
    pub character_map: HashMap<String, String>,
    pub token_map: HashMap<String, String>,
}

/// TextNormalizer struct that holds character and token mappings
pub struct TextNormalizer {
    pub character_map: HashMap<String, String>,
    pub token_map: HashMap<String, String>,
}

impl TextNormalizer {
    /// Create a new TextNormalizer with the given rules
    pub fn new(rules: Rules) -> Self {
        Self {
            character_map: rules.character_map,
            token_map: rules.token_map,
        }
    }
    
    /// Load rules from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rules: Rules = serde_json::from_reader(reader)?;
        Ok(Self::new(rules))
    }
}

/// Load normalization rules from the embedded JSON file
pub fn load_rules() -> TextNormalizer {
    let rules_json = include_str!("tn_rules.json");
    let rules: Rules = serde_json::from_str(rules_json)
        .expect("Failed to parse embedded rules JSON");
    TextNormalizer::new(rules)
}

// Singleton instance for global access
use std::sync::OnceLock;

static TEXT_NORMALIZER: OnceLock<TextNormalizer> = OnceLock::new();

/// Get the global TextNormalizer instance
pub fn get_normalizer() -> &'static TextNormalizer {
    TEXT_NORMALIZER.get_or_init(|| load_rules())
}
