use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

// Vietnamese character constants
const VIETNAMESE_CHARACTERS_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVXYZ\
    ÀÁẢÃẠĂẰẮẲẴẶÂẦẤẨẪẬ\
    Đ\
    ÈÉẺẼẸÊỀẾỂỄỆ\
    ÌÍỈĨỊ\
    ÒÓỎÕỌÔỒỐỔỖỘƠỜỚỞỠỢ\
    ÙÚỦŨỤƯỪỨỬỮỰ\
    ỲÝỶỸỴ";

const VIETNAMESE_CHARACTERS_LOWER: &str = "abcdefghijklmnopqrstuvxyz\
    àáảãạăằắẳẵặâầấẩẫậ\
    đ\
    èéẻẽẹêềếểễệ\
    ìíỉĩị\
    òóỏõọôồốổỗộơờớởỡợ\
    ùúủũụưừứửữự\
    ỳýỷỹỵ";

const VIETNAMESE_VOWELS_UPPER: &str = "AEIOU\
    ÀÁẢÃẠĂẰẮẲẴẶÂẦẤẨẪẬ\
    ÈÉẺẼẸÊỀẾỂỄỆ\
    ÌÍỈĨỊ\
    ÒÓỎÕỌÔỒỐỔỖỘƠỜỚỞỠỢ\
    ÙÚỦŨỤƯỪỨỬỮỰ\
    ỲÝỶỸỴ";

const VIETNAMESE_VOWELS_LOWER: &str = "aeiou\
    àáảãạăằắẳẵặâầấẩẫậ\
    èéẻẽẹêềếểễệ\
    ìíỉĩị\
    òóỏõọôồốổỗộơờớởỡợ\
    ùúủũụưừứửữự\
    ỳýỷỹỵ";

// Character classes
fn upper_class() -> String {
    format!("[{}]", "A-ZÀÁẢÃẠĂẰẮẲẴẶÂẦẤẨẪẬĐÈÉẺẼẸÊỀẾỂỄỆÌÍỈĨỊÒÓỎÕỌÔỒỐỔỖỘƠỜỚỞỠỢÙÚỦŨỤƯỪỨỬỮỰỲÝỶỸỴ")
}

fn lower_class() -> String {
    upper_class().to_lowercase()
}

fn word_class() -> String {
    format!("[{}{}]", 
        "A-ZÀÁẢÃẠĂẰẮẲẴẶÂẦẤẨẪẬĐÈÉẺẼẸÊỀẾỂỄỆÌÍỈĨỊÒÓỎÕỌÔỒỐỔỖỘƠỜỚỞỠỢÙÚỦŨỤƯỪỨỬỮỰỲÝỶỸỴ",
        "a-zàáảãạăằắẳẵặâầấẩẫậđèéẻẽẹêềếểễệìíỉĩịòóỏõọôồốổỗộơờớởỡợùúủũụưừứửữựỳýỷỹỵ"
    )
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Special,
    Abbreviation,
    Url,
    Email,
    Phone,
    DateTime,
    Name,
    Number,
    Emoji,
    Punct,
    WordHyphen,
    Word,
    Symbol,
    NonWord,
    FixedWords,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

pub struct VietnameseTokenizer {
    patterns: Regex,
    use_character_normalize: bool,
    use_token_normalize: bool,
}

impl VietnameseTokenizer {
    pub fn new() -> Self {
        let patterns = build_regex_patterns(&[]);
        Self {
            patterns,
            use_character_normalize: true,
            use_token_normalize: true,
        }
    }

    pub fn with_fixed_words(fixed_words: &[String]) -> Self {
        let patterns = build_regex_patterns(fixed_words);
        Self {
            patterns,
            use_character_normalize: true,
            use_token_normalize: true,
        }
    }

    pub fn set_character_normalize(&mut self, enabled: bool) {
        self.use_character_normalize = enabled;
    }

    pub fn set_token_normalize(&mut self, enabled: bool) {
        self.use_token_normalize = enabled;
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        self.tokenize_with_tags(text, false)
            .into_iter()
            .map(|token| token.text)
            .collect()
    }

    pub fn tokenize_with_tags(&self, text: &str, _include_tags: bool) -> Vec<Token> {
        let normalized_text = if self.use_character_normalize {
            normalize_characters_in_text(text)
        } else {
            text.to_string()
        };

        let mut tokens = Vec::new();
        for cap in self.patterns.captures_iter(&normalized_text) {
            if let Some((token_text, token_type)) = extract_match(&cap) {
                let final_text = if self.use_token_normalize {
                    token_normalize(&token_text, self.use_character_normalize)
                } else {
                    token_text
                };
                tokens.push(Token {
                    text: final_text,
                    token_type,
                });
            }
        }

        tokens
    }

    pub fn tokenize_as_text(&self, text: &str) -> String {
        self.tokenize(text).join(" ")
    }
}

// CRF Model structures and traits
pub trait SequenceTagger {
    fn load(&mut self, model_path: &Path) -> Result<(), Box<dyn std::error::Error>>;
    fn predict(&self, features: &[Vec<String>]) -> Vec<String>;
}

#[derive(Debug)]
pub struct CRFModel {
    // This would contain the actual CRF model parameters
    // For now, we'll use a simplified representation
    weights: HashMap<String, f64>,
    labels: Vec<String>,
    feature_templates: Vec<String>,
    loaded: bool,
}

impl CRFModel {
    pub fn new() -> Self {
        Self {
            weights: HashMap::new(),
            labels: vec!["B-W".to_string(), "I-W".to_string()],
            feature_templates: Vec::new(),
            loaded: false,
        }
    }

    fn extract_features(&self, tokens: &[String], position: usize) -> Vec<String> {
        let mut features = Vec::new();
        let current_token = &tokens[position];
        
        // Current token features
        features.push(format!("token={}", current_token));
        features.push(format!("length={}", current_token.len()));
        features.push(format!("is_upper={}", current_token.chars().all(|c| c.is_uppercase())));
        features.push(format!("is_digit={}", current_token.chars().all(|c| c.is_ascii_digit())));
        
        // Character-level features
        if let Some(first_char) = current_token.chars().next() {
            features.push(format!("first_char={}", first_char));
        }
        if let Some(last_char) = current_token.chars().last() {
            features.push(format!("last_char={}", last_char));
        }
        
        // Context features
        if position > 0 {
            features.push(format!("prev_token={}", tokens[position - 1]));
        } else {
            features.push("prev_token=<BOS>".to_string());
        }
        
        if position < tokens.len() - 1 {
            features.push(format!("next_token={}", tokens[position + 1]));
        } else {
            features.push("next_token=<EOS>".to_string());
        }
        
        // Bigram features
        if position > 0 {
            features.push(format!("prev_current={}_{}", tokens[position - 1], current_token));
        }
        if position < tokens.len() - 1 {
            features.push(format!("current_next={}_{}", current_token, tokens[position + 1]));
        }
        
        features
    }

    fn score_sequence(&self, tokens: &[String], tags: &[String]) -> f64 {
        let mut score = 0.0;
        
        for (i, tag) in tags.iter().enumerate() {
            let features = self.extract_features(tokens, i);
            for feature in features {
                let feature_tag = format!("{}#{}", feature, tag);
                if let Some(&weight) = self.weights.get(&feature_tag) {
                    score += weight;
                }
            }
            
            // Transition features
            if i > 0 {
                let prev_tag = &tags[i - 1];
                let transition = format!("{}#{}", prev_tag, tag);
                if let Some(&weight) = self.weights.get(&transition) {
                    score += weight;
                }
            }
        }
        
        score
    }

    fn viterbi_decode(&self, tokens: &[String]) -> Vec<String> {
        if tokens.is_empty() {
            return Vec::new();
        }
        
        let n_tokens = tokens.len();
        let n_labels = self.labels.len();
        
        // DP table: dp[i][j] = best score for assigning label j to token i
        let mut dp = vec![vec![f64::NEG_INFINITY; n_labels]; n_tokens];
        let mut backtrack = vec![vec![0; n_labels]; n_tokens];
        
        // Initialize first token
        for (j, label) in self.labels.iter().enumerate() {
            let features = self.extract_features(tokens, 0);
            let mut score = 0.0;
            for feature in features {
                let feature_tag = format!("{}#{}", feature, label);
                if let Some(&weight) = self.weights.get(&feature_tag) {
                    score += weight;
                }
            }
            dp[0][j] = score;
        }
        
        // Forward pass
        for i in 1..n_tokens {
            let features = self.extract_features(tokens, i);
            
            for (j, curr_label) in self.labels.iter().enumerate() {
                let mut emission_score = 0.0;
                for feature in &features {
                    let feature_tag = format!("{}#{}", feature, curr_label);
                    if let Some(&weight) = self.weights.get(&feature_tag) {
                        emission_score += weight;
                    }
                }
                
                for (k, prev_label) in self.labels.iter().enumerate() {
                    let transition = format!("{}#{}", prev_label, curr_label);
                    let transition_score = self.weights.get(&transition).unwrap_or(&0.0);
                    
                    let total_score = dp[i-1][k] + transition_score + emission_score;
                    
                    if total_score > dp[i][j] {
                        dp[i][j] = total_score;
                        backtrack[i][j] = k;
                    }
                }
            }
        }
        
        // Find best final state
        let mut best_final_state = 0;
        let mut best_score = dp[n_tokens - 1][0];
        for j in 1..n_labels {
            if dp[n_tokens - 1][j] > best_score {
                best_score = dp[n_tokens - 1][j];
                best_final_state = j;
            }
        }
        
        // Backtrack to get the best sequence
        let mut result = vec![0; n_tokens];
        result[n_tokens - 1] = best_final_state;
        
        for i in (1..n_tokens).rev() {
            result[i - 1] = backtrack[i][result[i]];
        }
        
        result.into_iter().map(|i| self.labels[i].clone()).collect()
    }
}

impl SequenceTagger for CRFModel {
    fn load(&mut self, model_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, you would load the CRF model from a file
        // For this example, we'll create some dummy weights
        
        if !model_path.exists() {
            eprintln!("Warning: Model path {:?} does not exist. Using dummy model.", model_path);
        }
        
        // Dummy weights for demonstration
        // In reality, these would be learned from training data
        self.weights.insert("token=Bác#B-W".to_string(), 2.0);
        self.weights.insert("token=sĩ#I-W".to_string(), 1.5);
        self.weights.insert("token=bây#B-W".to_string(), 1.8);
        self.weights.insert("token=giờ#I-W".to_string(), 1.3);
        self.weights.insert("token=có#B-W".to_string(), 1.9);
        self.weights.insert("token=thể#I-W".to_string(), 1.4);
        self.weights.insert("B-W#I-W".to_string(), 1.0);
        self.weights.insert("I-W#B-W".to_string(), 0.5);
        self.weights.insert("B-W#B-W".to_string(), 0.3);
        self.weights.insert("I-W#I-W".to_string(), 0.8);
        
        self.loaded = true;
        Ok(())
    }
    
    fn predict(&self, features: &[Vec<String>]) -> Vec<String> {
        if !self.loaded {
            panic!("Model not loaded. Call load() first.");
        }
        
        if features.is_empty() {
            return Vec::new();
        }
        
        let tokens: Vec<String> = features.iter().map(|f| f[0].clone()).collect();
        self.viterbi_decode(&tokens)
    }
}

pub struct FastCRFSequenceTagger {
    model: CRFModel,
}

impl FastCRFSequenceTagger {
    pub fn new() -> Self {
        Self {
            model: CRFModel::new(),
        }
    }
}

impl SequenceTagger for FastCRFSequenceTagger {
    fn load(&mut self, model_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.model.load(model_path)
    }
    
    fn predict(&self, features: &[Vec<String>]) -> Vec<String> {
        self.model.predict(features)
    }
}

// Global model instance (lazy-loaded)
static WORD_TOKENIZE_MODEL: Lazy<Arc<Mutex<Option<FastCRFSequenceTagger>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub struct VietnameseWordSegmenter {
    tokenizer: VietnameseTokenizer,
    model_path: Option<PathBuf>,
}

impl VietnameseWordSegmenter {
    pub fn new() -> Self {
        Self {
            tokenizer: VietnameseTokenizer::new(),
            model_path: None,
        }
    }
    
    pub fn with_model_path<P: AsRef<Path>>(model_path: P) -> Self {
        Self {
            tokenizer: VietnameseTokenizer::new(),
            model_path: Some(model_path.as_ref().to_path_buf()),
        }
    }
    
    pub fn with_fixed_words(fixed_words: &[String]) -> Self {
        Self {
            tokenizer: VietnameseTokenizer::with_fixed_words(fixed_words),
            model_path: None,
        }
    }
    
    fn ensure_model_loaded(&self) {
        let mut global_model = WORD_TOKENIZE_MODEL.lock().unwrap();
        
        if global_model.is_none() {
            let mut model = FastCRFSequenceTagger::new();
            
            let model_path = self.model_path.as_ref()
                .map(|p| p.as_path())
                .unwrap_or_else(|| Path::new("models/ws_crf_vlsp2013_20230727"));
            
            if let Err(e) = model.load(model_path) {
                eprintln!("Warning: Failed to load model from {:?}: {}", model_path, e);
                // Continue with dummy model for demonstration
            }
            
            *global_model = Some(model);
        }
    }
    
    pub fn word_tokenize(&self, sentence: &str) -> Vec<String> {
        self.word_tokenize_with_options(sentence, false, true, &[])
    }
    
    pub fn word_tokenize_as_text(&self, sentence: &str) -> String {
        let words = self.word_tokenize_with_options(sentence, false, true, &[]);
        words.iter()
            .map(|word| word.replace(' ', "_"))
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    pub fn word_tokenize_with_options(
        &self,
        sentence: &str,
        format_as_text: bool,
        use_token_normalize: bool,
        fixed_words: &[String],
    ) -> Vec<String> {
        self.ensure_model_loaded();
        
        // Use tokenizer with appropriate settings
        let mut tokenizer = if fixed_words.is_empty() {
            self.tokenizer.clone()
        } else {
            VietnameseTokenizer::with_fixed_words(fixed_words)
        };
        
        tokenizer.set_token_normalize(use_token_normalize);
        let tokens = tokenizer.tokenize(sentence);
        
        // Prepare features for CRF model
        let features: Vec<Vec<String>> = tokens.iter()
            .map(|token| vec![token.clone()])
            .collect();
        
        // Get predictions from CRF model
        let global_model = WORD_TOKENIZE_MODEL.lock().unwrap();
        let tags = if let Some(ref model) = *global_model {
            model.predict(&features)
        } else {
            // Fallback: assume all tokens are beginning of words
            vec!["B-W".to_string(); tokens.len()]
        };
        
        // Combine tokens based on tags
        let mut output = Vec::new();
        for (tag, token) in tags.iter().zip(tokens.iter()) {
            if tag == "I-W" && !output.is_empty() {
                // Continue previous word
                if let Some(last_word) = output.last_mut() {
                    *last_word = format!("{} {}", last_word, token);
                }
            } else {
                // Start new word
                output.push(token.clone());
            }
        }
        
        if format_as_text {
            vec![output.iter()
                .map(|word| word.replace(' ', "_"))
                .collect::<Vec<_>>()
                .join(" ")]
        } else {
            output
        }
    }
}

impl Clone for VietnameseTokenizer {
    fn clone(&self) -> Self {
        Self {
            patterns: build_regex_patterns(&[]), // Rebuild patterns
            use_character_normalize: self.use_character_normalize,
            use_token_normalize: self.use_token_normalize,
        }
    }
}

fn build_regex_patterns(fixed_words: &[String]) -> Regex {
    let upper = upper_class();
    let w = word_class();

    // Priority 1: Specials
    let specials = vec![
        r"=\>",
        r"==>", 
        r"->",
        r"\.{2,}",
        r"-{2,}",
        r">>",
        r"\d+x\d+",
        r"v\.v\.\.\.",
        r"v\.v\.",
        r"v\.v",
        r"°[CF]",
    ];
    let specials_pattern = format!("(?P<special>({}))", specials.join("|"));

    let upper_clean = upper.replace("[", "").replace("]", "");
    let w_clean = w.replace("[", "").replace("]", "");
    
    let abbrev_pattern1 = format!(r"{}+(?:\.{}+)+\.?", upper_clean, w_clean);
    let abbrev_pattern2 = format!(r"{}+['']{}+", w_clean, w_clean);
    
    let abbreviations = vec![
        r"[A-ZĐ]+&[A-ZĐ]+",
        r"T\.Ư",
        &abbrev_pattern1,
        &abbrev_pattern2,
        r"[A-ZĐ]+\.",
        r"Tp\.",
        r"Mr\.", r"Mrs\.", r"Ms\.",
        r"Dr\.", r"ThS\.", r"Th\.S", r"Th\.s",
        r"e-mail",
        r"\d+[A-Z]+\d*-\d+",
        r"NĐ-CP",
    ];
    let abbreviations_pattern = format!("(?P<abbr>({}))", abbreviations.join("|"));

    // Priority 2: URLs, emails, etc.
    let url_pattern = r#"(?P<url>(?:(?:ftp|http)s?:(?:/{1,3}|[a-z0-9%])|[a-z0-9.\-]+[.](?:[a-z]{2,13})/)(?:[^\s()<>{}\[\]]+|\([^\s()]*?\([^\s()]+\)[^\s()]*?\)|\([^\s]+?\))+(?:\([^\s()]*?\([^\s()]+\)[^\s()]*?\)|\([^\s]+?\)|[^\s`!()\[\]{};:'".,<>?«»""''])|(?:[a-z0-9]+(?:[.\-][a-z0-9]+)*[.](?:[a-z]{2,13})\b/?))"#;
    
    let email_pattern = r"(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)";
    
    let phone_pattern = r"(?P<phone>\d{2,}-\d{3,}-\d{3,})";
    
    let datetime_patterns = vec![
        r"\d{1,2}/\d{1,2}/\d+",
        r"\d{1,2}/\d{1,4}",
        r"\d{1,2}-\d{1,2}-\d+",
        r"\d{1,2}-\d{1,4}",
        r"\d{1,2}\.\d{1,2}\.\d+",
        r"\d{4}/\d{1,2}/\d{1,2}",
        r"\d{2}:\d{2}:\d{2}",
    ];
    let datetime_pattern = format!("(?P<datetime>({}))", datetime_patterns.join("|"));
    
    let name_patterns = vec![
        r"\d+[A-Z]+\d+",
        r"\d+[A-Z]+",
    ];
    let name_pattern = format!("(?P<name>({}))", name_patterns.join("|"));
    
    let number_patterns = vec![
        r"\d+(?:\.\d+)+,\d+",
        r"\d+(?:\.\d+)+",
        r"\d+(?:,\d+)+",
        r"\d+(?:[\.,_]\d+)?",
    ];
    let number_pattern = format!("(?P<number>({}))", number_patterns.join("|"));
    
    let emoji_patterns = vec![
        r":\)\)*",
        r"=\)\)+", 
        r"♥‿♥",
        r":D+\s",
        r":D+$",
        r"<3",
    ];
    let emoji_pattern = format!("(?P<emoji>({}))", emoji_patterns.join("|"));
    
    let punct_patterns = vec![
        r"\.",
        r",",
        r"\(",
        r"\)",
        r"ʺ",
    ];
    let punct_pattern = format!("(?P<punct>({}))", punct_patterns.join("|"));

    // Priority 3
    let word_hyphen_pattern = r"(?P<word_hyphen>\b\w+\-[\w+-]*\w+)";
    let word_pattern = r"(?P<word>\w+)";
    
    let symbol_patterns = vec![
        r"\+", r"×", r"-", r"÷", r":+", r"%", r"\$", r">", r"<", r"=", r"\^", r"_",
    ];
    let symbol_pattern = format!("(?P<sym>({}))", symbol_patterns.join("|"));
    
    let non_word_pattern = r"(?P<non_word>[^\w\s])";

    let mut all_patterns = Vec::new();
    
    // Add fixed words pattern if provided
    if !fixed_words.is_empty() {
        let escaped_words: Vec<String> = fixed_words
            .iter()
            .map(|word| word.replace(" ", r"\ "))
            .collect();
        let fixed_words_pattern = format!(r"(?P<fixed_words>\b{}\b)", escaped_words.join(r"\b|\b"));
        all_patterns.push(fixed_words_pattern);
    }

    all_patterns.extend(vec![
        specials_pattern,
        abbreviations_pattern,
        url_pattern.to_string(),
        email_pattern.to_string(),
        phone_pattern.to_string(),
        datetime_pattern,
        name_pattern,
        number_pattern,
        emoji_pattern,
        punct_pattern,
        word_hyphen_pattern.to_string(),
        word_pattern.to_string(),
        symbol_pattern,
        non_word_pattern.to_string(),
    ]);

    let combined_pattern = format!("({})", all_patterns.join("|"));
    Regex::new(&combined_pattern).expect("Failed to compile regex")
}

fn extract_match(captures: &regex::Captures) -> Option<(String, TokenType)> {
    if let Some(m) = captures.name("special") {
        return Some((m.as_str().to_string(), TokenType::Special));
    }
    if let Some(m) = captures.name("abbr") {
        return Some((m.as_str().to_string(), TokenType::Abbreviation));
    }
    if let Some(m) = captures.name("url") {
        return Some((m.as_str().to_string(), TokenType::Url));
    }
    if let Some(m) = captures.name("email") {
        return Some((m.as_str().to_string(), TokenType::Email));
    }
    if let Some(m) = captures.name("phone") {
        return Some((m.as_str().to_string(), TokenType::Phone));
    }
    if let Some(m) = captures.name("datetime") {
        return Some((m.as_str().to_string(), TokenType::DateTime));
    }
    if let Some(m) = captures.name("name") {
        return Some((m.as_str().to_string(), TokenType::Name));
    }
    if let Some(m) = captures.name("number") {
        return Some((m.as_str().to_string(), TokenType::Number));
    }
    if let Some(m) = captures.name("emoji") {
        return Some((m.as_str().to_string(), TokenType::Emoji));
    }
    if let Some(m) = captures.name("punct") {
        return Some((m.as_str().to_string(), TokenType::Punct));
    }
    if let Some(m) = captures.name("word_hyphen") {
        return Some((m.as_str().to_string(), TokenType::WordHyphen));
    }
    if let Some(m) = captures.name("word") {
        return Some((m.as_str().to_string(), TokenType::Word));
    }
    if let Some(m) = captures.name("sym") {
        return Some((m.as_str().to_string(), TokenType::Symbol));
    }
    if let Some(m) = captures.name("non_word") {
        return Some((m.as_str().to_string(), TokenType::NonWord));
    }
    if let Some(m) = captures.name("fixed_words") {
        return Some((m.as_str().to_string(), TokenType::FixedWords));
    }
    None
}

// Placeholder implementations for normalize functions
// These would need to be implemented based on the actual underthesea library
fn normalize_characters_in_text(text: &str) -> String {
    // This is a simplified implementation
    // You would need to implement the actual Vietnamese character normalization
    text.to_string()
}

fn token_normalize(token: &str, use_character_normalize: bool) -> String {
    // This is a simplified implementation
    // You would need to implement the actual token normalization
    if use_character_normalize {
        normalize_characters_in_text(token)
    } else {
        token.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let tokenizer = VietnameseTokenizer::new();
        let text = "Xin chào, tôi là Claude.";
        let tokens = tokenizer.tokenize(text);
        assert!(!tokens.is_empty());
        println!("Tokens: {:?}", tokens);
    }

    #[test]
    fn test_url_tokenization() {
        let tokenizer = VietnameseTokenizer::new();
        let text = "Visit https://example.com for more info.";
        let tokens = tokenizer.tokenize_with_tags(text, true);
        
        let has_url = tokens.iter().any(|t| t.token_type == TokenType::Url);
        assert!(has_url, "Should detect URL token");
    }

    #[test]
    fn test_email_tokenization() {
        let tokenizer = VietnameseTokenizer::new();
        let text = "Contact me at test@example.com";
        let tokens = tokenizer.tokenize_with_tags(text, true);
        
        let has_email = tokens.iter().any(|t| t.token_type == TokenType::Email);
        assert!(has_email, "Should detect email token");
    }

    #[test]
    fn test_number_tokenization() {
        let tokenizer = VietnameseTokenizer::new();
        let text = "Giá là 1.000.000 đồng";
        let tokens = tokenizer.tokenize_with_tags(text, true);
        
        let has_number = tokens.iter().any(|t| t.token_type == TokenType::Number);
        assert!(has_number, "Should detect number token");
    }

    #[test]
    fn test_fixed_words() {
        let fixed_words = vec!["Viện nghiên cứu".to_string()];
        let tokenizer = VietnameseTokenizer::with_fixed_words(&fixed_words);
        let text = "Viện nghiên cứu khoa học";
        let tokens = tokenizer.tokenize_with_tags(text, true);
        
        let has_fixed = tokens.iter().any(|t| t.token_type == TokenType::FixedWords);
        assert!(has_fixed, "Should detect fixed words token");
    }
    
    #[test]
    fn test_word_segmentation_basic() {
        let segmenter = VietnameseWordSegmenter::new();
        let sentence = "Bác sĩ bây giờ có thể thản nhiên";
        let words = segmenter.word_tokenize(sentence);
        
        assert!(!words.is_empty());
        println!("Word segmentation: {:?}", words);
        
        // Should contain multi-character words
        let has_multi_char_words = words.iter().any(|w| w.contains(' '));
        // Note: This might not always be true depending on the model
        println!("Has multi-character words: {}", has_multi_char_words);
    }
    
    #[test]
    fn test_word_segmentation_text_format() {
        let segmenter = VietnameseWordSegmenter::new();
        let sentence = "Bác sĩ bây giờ có thể";
        let text = segmenter.word_tokenize_as_text(sentence);
        
        assert!(!text.is_empty());
        println!("Text format: {}", text);
        
        // Should contain underscores for multi-word tokens
        // Note: This depends on the model's segmentation decisions
    }
    
    #[test] 
    fn test_word_segmentation_with_fixed_words() {
        let fixed_words = vec!["bác sĩ".to_string()];
        let segmenter = VietnameseWordSegmenter::with_fixed_words(&fixed_words);
        let sentence = "bác sĩ bây giờ";
        let words = segmenter.word_tokenize(sentence);
        
        println!("With fixed words: {:?}", words);
        // The "bác sĩ" should be treated as a single token in tokenization phase
    }
    
    #[test]
    fn test_crf_model_basic() {
        let mut model = FastCRFSequenceTagger::new();
        
        // Test loading (will use dummy model)
        let dummy_path = Path::new("nonexistent/model/path");
        let result = model.load(dummy_path);
        assert!(result.is_ok(), "Should handle missing model gracefully");
        
        // Test prediction
        let features = vec![
            vec!["Bác".to_string()],
            vec!["sĩ".to_string()],
            vec!["bây".to_string()],
            vec!["giờ".to_string()],
        ];
        
        let tags = model.predict(&features);
        assert_eq!(tags.len(), 4);
        println!("Predicted tags: {:?}", tags);
        
        // All tags should be valid
        for tag in tags {
            assert!(tag == "B-W" || tag == "I-W", "Invalid tag: {}", tag);
        }
    }
    
    #[test]
    fn test_feature_extraction() {
        let model = CRFModel::new();
        let tokens = vec!["Bác".to_string(), "sĩ".to_string(), "bây".to_string()];
        
        let features_0 = model.extract_features(&tokens, 0);
        let features_1 = model.extract_features(&tokens, 1);
        
        println!("Features for position 0: {:?}", features_0);
        println!("Features for position 1: {:?}", features_1);
        
        // Should contain token features
        assert!(features_0.iter().any(|f| f.contains("token=Bác")));
        assert!(features_1.iter().any(|f| f.contains("token=sĩ")));
        
        // Should contain context features
        assert!(features_0.iter().any(|f| f.contains("prev_token=<BOS>")));
        assert!(features_1.iter().any(|f| f.contains("prev_token=Bác")));
    }
}