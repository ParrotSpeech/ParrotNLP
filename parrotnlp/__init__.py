# This is the public API of parrotnlp
# Word tokenization functionality
try:
    from .word_tokenize import word_tokenize, VietnameseWordSegmenter, VietnameseTokenizer
    # Import text normalization functions from the Rust extension
    from ._parrotnlp import (
        text_normalize_py as text_normalize,
        token_normalize_py as token_normalize, 
        normalize_characters_in_text_py as normalize_characters_in_text
    )
except ImportError:
    # Fallback if the Rust extension is not available
    def word_tokenize(text, output_format="list", fixed_words=None):
        raise ImportError("word_tokenize functionality requires the Rust extension to be built")
    
    def text_normalize(text, tokenizer="underthesea"):
        raise ImportError("text_normalize functionality requires the Rust extension to be built")
    
    def token_normalize(token, use_character_normalize=True):
        raise ImportError("token_normalize functionality requires the Rust extension to be built")
    
    def normalize_characters_in_text(text):
        raise ImportError("normalize_characters_in_text functionality requires the Rust extension to be built")
    
    class VietnameseWordSegmenter:
        def __init__(self, fixed_words=None):
            raise ImportError("VietnameseWordSegmenter requires the Rust extension to be built")
    
    class VietnameseTokenizer:
        def __init__(self, fixed_words=None):
            raise ImportError("VietnameseTokenizer requires the Rust extension to be built")

__version__ = "0.1.0"
