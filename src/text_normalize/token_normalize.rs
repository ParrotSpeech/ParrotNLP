use crate::text_normalize::character_normalize::normalize_characters_in_text;
use crate::text_normalize::text_normalizer::get_normalizer;

/// Normalize a single token
/// 
/// # Arguments
/// 
/// * `token` - The token to normalize
/// * `use_character_normalize` - Whether to apply character normalization
/// 
/// # Returns
/// 
/// The normalized token
pub fn token_normalize(token: &str, use_character_normalize: bool) -> String {
    // Skip normalization for tokens longer than 6 characters
    if token.len() > 6 {
        return token.to_string();
    }
    
    let mut normalized_token = token.to_string();
    
    // Apply character normalization if requested
    if use_character_normalize {
        normalized_token = normalize_characters_in_text(&normalized_token);
    }
    
    // Apply token mapping if exists
    let normalizer = get_normalizer();
    if let Some(mapped_token) = normalizer.token_map.get(&normalized_token) {
        return mapped_token.clone();
    }
    
    normalized_token
}
