pub mod text_normalizer;
pub mod character_normalize;
pub mod token_normalize;

pub use text_normalizer::{TextNormalizer, load_rules};
pub use character_normalize::{character_normalize, utf8_normalize, normalize_characters_in_text};
pub use token_normalize::token_normalize;

use crate::word_tokenize::VietnameseTokenizer;

/// Text normalization function
/// 
/// # Arguments
/// 
/// * `text` - The text to normalize
/// * `tokenizer` - The tokenizer to use: "space" or "underthesea"
/// 
/// # Returns
/// 
/// The normalized text
pub fn text_normalize(text: &str, tokenizer: &str) -> String {
    let tokens = match tokenizer {
        "underthesea" => {
            let tokenizer = VietnameseTokenizer::new();
            tokenizer.tokenize(text)
        }
        _ => {
            // Default to space tokenization
            text.split_whitespace().map(|s| s.to_string()).collect()
        }
    };
    
    let normalized_tokens: Vec<String> = tokens
        .iter()
        .map(|token| token_normalize(token, true))
        .collect();
    
    normalized_tokens.join(" ")
}
