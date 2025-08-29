# This is the public API of parrotnlp
# Word tokenization functionality
try:
    from .word_tokenize import word_tokenize, VietnameseWordSegmenter, VietnameseTokenizer
except ImportError:
    # Fallback if the Rust extension is not available
    def word_tokenize(text, format="list", fixed_words=None):
        raise ImportError("word_tokenize functionality requires the Rust extension to be built")
    
    class VietnameseWordSegmenter:
        def __init__(self, fixed_words=None):
            raise ImportError("VietnameseWordSegmenter requires the Rust extension to be built")
    
    class VietnameseTokenizer:
        def __init__(self, fixed_words=None):
            raise ImportError("VietnameseTokenizer requires the Rust extension to be built")

__version__ = "0.1.0"
