use regex::Regex;
use std::collections::HashMap;
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

    pub fn tokenize_with_tags(&self, text: &str, include_tags: bool) -> Vec<Token> {
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

    let abbreviations = vec![
        r"[A-ZĐ]+&[A-ZĐ]+",
        r"T\.Ư",
        &format!(r"{}+(?:\.{}+)+\.?", upper.replace("[", "").replace("]", ""), w.replace("[", "").replace("]", "")),
        &format!(r"{}+['']{}+", w.replace("[", "").replace("]", ""), w.replace("[", "").replace("]", "")),
        r"[A-ZĐ]+\.(?!$)",
        r"Tp\.",
        r"Mr\.", r"Mrs\.", r"Ms\.",
        r"Dr\.", r"ThS\.", r"Th\.S", r"Th\.s",
        r"e-mail",
        r"\d+[A-Z]+\d*-\d+",
        r"NĐ-CP",
    ];
    let abbreviations_pattern = format!("(?P<abbr>({}))", abbreviations.join("|"));

    // Priority 2: URLs, emails, etc.
    let url_pattern = r#"(?P<url>(?:(?:ftp|http)s?:(?:/{1,3}|[a-z0-9%])|[a-z0-9.\-]+[.](?:[a-z]{2,13})/)(?:[^\s()<>{}\[\]]+|\([^\s()]*?\([^\s()]+\)[^\s()]*?\)|\([^\s]+?\))+(?:\([^\s()]*?\([^\s()]+\)[^\s()]*?\)|\([^\s]+?\)|[^\s`!()\[\]{};:'".,<>?«»""''])|(?:(?<!@)[a-z0-9]+(?:[.\-][a-z0-9]+)*[.](?:[a-z]{2,13})\b/?(?!@)))"#;
    
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
        r":D+(?=\s)",
        r":D+(?=$)",
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
    let word_hyphen_pattern = r"(?P<word_hyphen>(?<=\b)\w+\-[\w+-]*\w+)";
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
}