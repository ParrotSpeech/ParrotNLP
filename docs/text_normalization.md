# Text Normalization in ParrotNLP

This document describes the text normalization functionality that has been migrated from Python to Rust.

## Overview

The text normalization module provides functionality to normalize Vietnamese text by:
1. **Character normalization**: Converting non-standard characters to standard equivalents (e.g., `ð` → `đ`)
2. **Token normalization**: Converting non-standard token forms to standard equivalents (e.g., `nghàn` → `ngàn`)
3. **Unicode normalization**: Applying Unicode NFC normalization for consistent text representation

## API Functions

### `text_normalize(text: str, tokenizer: str = "underthesea") -> str`

Normalizes an entire text by tokenizing it and applying both character and token normalization.

**Parameters:**
- `text`: The input text to normalize
- `tokenizer`: The tokenization method to use (`"underthesea"` or `"space"`)

**Returns:** The normalized text as a string

**Example:**
```python
import parrotnlp

text = "Tôi thích nghàn con mèo. Ð là ký tự đặc biệt."
normalized = parrotnlp.text_normalize(text, "space")
print(normalized)  # "Tôi thích ngàn con mèo. Đ là ký tự đặc biệt."
```

### `token_normalize(token: str, use_character_normalize: bool = True) -> str`

Normalizes a single token.

**Parameters:**
- `token`: The token to normalize
- `use_character_normalize`: Whether to apply character normalization

**Returns:** The normalized token

**Behavior:**
- Tokens longer than 6 characters are returned unchanged
- Character normalization is applied first (if enabled)
- Token mapping is applied if the token exists in the mapping table

**Example:**
```python
import parrotnlp

token = parrotnlp.token_normalize("nghàn", True)
print(token)  # "ngàn"
```

### `normalize_characters_in_text(text: str) -> str`

Normalizes characters in text by applying Unicode NFC normalization and character mapping.

**Parameters:**
- `text`: The text to normalize

**Returns:** The text with normalized characters

**Example:**
```python
import parrotnlp

text = parrotnlp.normalize_characters_in_text("ðÐ")
print(text)  # "đĐ"
```

## Implementation Details

### Character Mapping
The character normalization uses a predefined mapping table that converts:
- `ð` → `đ`  
- `Ð` → `Đ`

### Token Mapping
The token normalization uses a comprehensive mapping table with 2,904 token mappings, including common Vietnamese word variations and corrections.

### Performance
The Rust implementation provides:
- **Memory efficiency**: Static compilation with embedded rule data
- **Speed**: No runtime file loading or parsing
- **Thread safety**: All functions are thread-safe

### Data Source
The normalization rules are loaded from `tn_rules_2023_07_14.bin` (converted to JSON format for Rust consumption).

## Migration Notes

The Rust implementation maintains full compatibility with the original Python implementation:
- Same function signatures (with Python bindings)
- Same normalization behavior
- Same rule data

Key improvements:
- Faster execution (compiled Rust vs interpreted Python)
- No external Python dependencies (joblib, underthesea for basic tokenization)
- Better memory management
- Thread safety

## Usage Examples

### Basic Text Normalization
```python
import parrotnlp

# Normalize with space tokenization
text = "Tôi thích nghàn ð mèo"
result = parrotnlp.text_normalize(text, "space")
print(result)  # "Tôi thích ngàn đ mèo"
```

### Character-only Normalization
```python
import parrotnlp

# Just normalize characters without token mapping
text = "ðÐ"
result = parrotnlp.normalize_characters_in_text(text)
print(result)  # "đĐ"
```

### Token Normalization with Control
```python
import parrotnlp

# Normalize token with character normalization
token1 = parrotnlp.token_normalize("nghàn", True)
print(token1)  # "ngàn"

# Normalize token without character normalization
token2 = parrotnlp.token_normalize("nghàn", False)
print(token2)  # "ngàn" (still normalized via token mapping)
```
