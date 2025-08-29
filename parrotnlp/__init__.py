# This is the public API of parrotnlp
from .core import Encoding as Encoding
from .model import encoding_for_model as encoding_for_model
from .model import encoding_name_for_model as encoding_name_for_model
from .registry import get_encoding as get_encoding
from .registry import list_encoding_names as list_encoding_names

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

__version__ = "0.11.0"
