use unicode_normalization::UnicodeNormalization;
use crate::text_normalize::text_normalizer::get_normalizer;

/// Normalize characters in text using the character map
pub fn character_normalize(text: &str) -> String {
    let normalizer = get_normalizer();
    let mut result = text.to_string();
    
    for (non_standard, standard) in &normalizer.character_map {
        result = result.replace(non_standard, standard);
    }
    
    result
}

/// Normalize UTF-8 text using Unicode NFC normalization
pub fn utf8_normalize(text: &str) -> String {
    text.nfc().collect()
}

/// Normalize characters in text by applying both UTF-8 and character normalization
pub fn normalize_characters_in_text(text: &str) -> String {
    let text = utf8_normalize(text);
    character_normalize(&text)
}
