# ParrotNLP

ParrotNLP now includes Vietnamese word tokenization functionality similar to underthesea. This feature provides fast and accurate Vietnamese word segmentation using Rust for optimal performance.

## Features

- **Fast Vietnamese word tokenization** using Rust implementation
- **Multiple output formats**: list of words or space-separated text with underscores
- **Fixed words support**: specify words that should be treated as single tokens
- **Class-based API**: flexible object-oriented interface
- **CRF model support**: can use trained Conditional Random Field models for better accuracy

## Installation

The word_tokenize functionality is included in the main ParrotNLP package:

```bash
pip install parrotnlp
```

## Quick Start

### Basic Usage

```python
from parrotnlp import word_tokenize

# Basic word tokenization
text = "Chàng trai 9X Quảng Trị khởi nghiệp từ nấm sò"
words = word_tokenize(text)
print(words)
# Output: ['Chàng trai', '9X', 'Quảng Trị', 'khởi nghiệp', 'từ', 'nấm', 'sò']
```

### Text Format

```python
# Get space-separated text with underscores for multi-word tokens
text = "Chàng trai 9X Quảng Trị khởi nghiệp từ nấm sò"
result = word_tokenize(text, format="text")
print(result)
# Output: "Chàng_trai 9X Quảng_Trị khởi_nghiệp từ nấm sò"
```

### Fixed Words

```python
# Specify words that should be treated as single tokens
text = "Viện Nghiên Cứu chiến lược quốc gia về học máy"
fixed_words = ["Viện Nghiên Cứu", "học máy"]
result = word_tokenize(text, fixed_words=fixed_words)
print(result)
# Output: ["Viện Nghiên Cứu", "chiến lược", "quốc gia", "về", "học máy"]
```

## Class-Based API

For more advanced usage, you can use the class-based API:

### VietnameseWordSegmenter

```python
from parrotnlp import VietnameseWordSegmenter

# Create a segmenter instance
segmenter = VietnameseWordSegmenter()

# Basic word tokenization
text = "Bác sĩ bây giờ có thể thản nhiên báo tin bệnh nhân bị ung thư"
words = segmenter.word_tokenize(text)
print(words)
# Output: ['Bác sĩ', 'bây giờ', 'có thể', 'thản nhiên', 'báo tin', 'bệnh nhân', 'bị', 'ung thư']

# Text format
text_result = segmenter.word_tokenize_as_text(text)
print(text_result)
# Output: "Bác_sĩ bây_giờ có_thể thản_nhiên báo_tin bệnh_nhân bị ung_thư"
```

### With Fixed Words

```python
# Create segmenter with fixed words
fixed_words = ["bác sĩ", "bệnh nhân"]
segmenter = VietnameseWordSegmenter(fixed_words=fixed_words)

text = "Bác sĩ bây giờ có thể thản nhiên báo tin bệnh nhân bị ung thư"
words = segmenter.word_tokenize(text)
print(words)
# Output: ['Bác sĩ', 'bây giờ', 'có thể', 'thản nhiên', 'báo tin', 'bệnh nhân', 'bị', 'ung thư']
```

## Advanced Features

### VietnameseTokenizer

For basic tokenization without word segmentation:

```python
from parrotnlp import VietnameseTokenizer

tokenizer = VietnameseTokenizer()
text = "Xin chào, tôi là Claude."
tokens = tokenizer.tokenize(text)
print(tokens)
# Output: ['Xin', 'chào', ',', 'tôi', 'là', 'Claude', '.']

# With token type tags
tokens_with_tags = tokenizer.tokenize_with_tags(text, include_tags=True)
print(tokens_with_tags)
# Output: [('Xin', 'TokenType::Word'), ('chào', 'TokenType::Word'), (',', 'TokenType::Punct'), ...]
```

## Model Support

The word tokenization system supports CRF (Conditional Random Field) models for improved accuracy. By default, it uses a dummy model, but you can provide your own trained model:

```python
# The system will automatically look for models in the models/ directory
# Currently supports: models/ws_crf_vlsp2013_20230727/
```

## Performance

The Rust implementation provides significant performance improvements over pure Python implementations:

- **Fast tokenization**: Optimized regex patterns and efficient algorithms
- **Memory efficient**: Minimal memory allocation during processing
- **Thread-safe**: Can be used in multi-threaded applications

## API Reference

### word_tokenize()

```python
word_tokenize(
    text: str,
    format: str = "list",
    fixed_words: Optional[List[str]] = None
) -> Union[List[str], str]
```

**Parameters:**
- `text`: Vietnamese text to tokenize
- `format`: Output format - "list" for list of words, "text" for space-separated string
- `fixed_words`: Optional list of fixed words that should be treated as single tokens

**Returns:**
- List of words if format="list"
- Space-separated string if format="text"

### VietnameseWordSegmenter

**Constructor:**
```python
VietnameseWordSegmenter(fixed_words: Optional[List[str]] = None)
```

**Methods:**
- `word_tokenize(sentence: str) -> List[str]`: Tokenize a Vietnamese sentence into words
- `word_tokenize_as_text(sentence: str) -> str`: Tokenize and return as text format

### VietnameseTokenizer

**Constructor:**
```python
VietnameseTokenizer(fixed_words: Optional[List[str]] = None)
```

**Methods:**
- `tokenize(text: str) -> List[str]`: Tokenize Vietnamese text into basic tokens
- `tokenize_with_tags(text: str, include_tags: bool = True) -> List[tuple]`: Tokenize with token type tags
- `tokenize_as_text(text: str) -> str`: Tokenize and return as space-separated string

## Examples

### Medical Text Processing

```python
from parrotnlp import word_tokenize

medical_text = "Bệnh nhân được chẩn đoán mắc bệnh tiểu đường type 2"
fixed_words = ["bệnh nhân", "tiểu đường", "type 2"]

words = word_tokenize(medical_text, fixed_words=fixed_words)
print(words)
# Output: ['Bệnh nhân', 'được', 'chẩn đoán', 'mắc', 'bệnh', 'tiểu đường', 'type 2']
```

### News Article Processing

```python
from parrotnlp import VietnameseWordSegmenter

segmenter = VietnameseWordSegmenter()
news_text = "Thủ tướng Chính phủ đã ký quyết định về việc phê duyệt kế hoạch phát triển kinh tế"

words = segmenter.word_tokenize(news_text)
print(words)
# Output: ['Thủ tướng', 'Chính phủ', 'đã', 'ký', 'quyết định', 'về', 'việc', 'phê duyệt', 'kế hoạch', 'phát triển', 'kinh tế']
```

## Comparison with underthesea

The ParrotNLP word_tokenize functionality provides a similar API to underthesea:

```python
# underthesea style
from underthesea import word_tokenize as underthesea_word_tokenize

# ParrotNLP style
from parrotnlp import word_tokenize

text = "Chàng trai 9X Quảng Trị khởi nghiệp từ nấm sò"

# Both should produce similar results
underthesea_result = underthesea_word_tokenize(text)
parrotnlp_result = word_tokenize(text)

print("underthesea:", underthesea_result)
print("parrotnlp:", parrotnlp_result)
```

## Error Handling

The word_tokenize functionality includes robust error handling:

- **Import errors**: Graceful fallback if Rust extension is not available
- **Regex compilation**: Handles complex Vietnamese character patterns
- **Model loading**: Continues with dummy model if trained model is not found

## Contributing

To contribute to the word_tokenize functionality:

1. The Rust implementation is in `src/word_tokenize/`
2. Python bindings are in `src/py.rs`
3. Python API is in `parrotnlp/word_tokenize.py`

## License

This functionality is part of ParrotNLP and is licensed under the MIT License.
