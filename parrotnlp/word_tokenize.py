"""
Vietnamese word tokenization module.

This module provides Vietnamese word tokenization functionality similar to underthesea.
"""

from typing import List, Optional, Union

try:
    from . import _parrotnlp
except ImportError:
    _parrotnlp = None


def word_tokenize(
    text: str,
    format: str = "list",
    fixed_words: Optional[List[str]] = None,
) -> Union[List[str], str]:
    """
    Tokenize Vietnamese text into words.
    
    Args:
        text: The Vietnamese text to tokenize
        format: Output format - "list" for list of words, "text" for space-separated string
        fixed_words: Optional list of fixed words that should be treated as single tokens
        
    Returns:
        List of words if format="list", space-separated string if format="text"
        
    Examples:
        >>> word_tokenize("Chàng trai 9X Quảng Trị khởi nghiệp từ nấm sò")
        ["Chàng trai", "9X", "Quảng Trị", "khởi nghiệp", "từ", "nấm", "sò"]
        
        >>> word_tokenize("Chàng trai 9X Quảng Trị khởi nghiệp từ nấm sò", format="text")
        "Chàng_trai 9X Quảng_Trị khởi_nghiệp từ nấm sò"
        
        >>> word_tokenize("Viện Nghiên Cứu chiến lược quốc gia về học máy", 
        ...              fixed_words=["Viện Nghiên Cứu", "học máy"])
        ["Viện Nghiên Cứu", "chiến lược", "quốc gia", "về", "học máy"]
    """
    if _parrotnlp is None:
        raise ImportError("word_tokenize functionality requires the Rust extension to be built")
    
    if fixed_words is None:
        fixed_words = []
    
    # Convert fixed_words to the format expected by Rust
    fixed_words_rust = [word for word in fixed_words]
    
    if format == "text":
        # Use the text format method
        segmenter = _parrotnlp.VietnameseWordSegmenterPy()
        return segmenter.word_tokenize_as_text(text)
    else:
        # Use the list format method
        if fixed_words:
            segmenter = _parrotnlp.VietnameseWordSegmenterPy.with_fixed_words(fixed_words_rust)
        else:
            segmenter = _parrotnlp.VietnameseWordSegmenterPy()
        return segmenter.word_tokenize(text)


class VietnameseWordSegmenter:
    """
    Vietnamese word segmenter class.
    
    This class provides methods for Vietnamese word segmentation with various options.
    """
    
    def __init__(self, fixed_words: Optional[List[str]] = None):
        """
        Initialize the word segmenter.
        
        Args:
            fixed_words: Optional list of fixed words that should be treated as single tokens
        """
        if _parrotnlp is None:
            raise ImportError("VietnameseWordSegmenter requires the Rust extension to be built")
        
        if fixed_words:
            self._segmenter = _parrotnlp.VietnameseWordSegmenterPy.with_fixed_words(fixed_words)
        else:
            self._segmenter = _parrotnlp.VietnameseWordSegmenterPy()
    
    def word_tokenize(self, sentence: str) -> List[str]:
        """
        Tokenize a Vietnamese sentence into words.
        
        Args:
            sentence: The Vietnamese sentence to tokenize
            
        Returns:
            List of words
            
        Examples:
            >>> segmenter = VietnameseWordSegmenter()
            >>> segmenter.word_tokenize("Bác sĩ bây giờ có thể thản nhiên")
            ["Bác sĩ", "bây giờ", "có thể", "thản nhiên"]
        """
        return self._segmenter.word_tokenize(sentence)
    
    def word_tokenize_as_text(self, sentence: str) -> str:
        """
        Tokenize a Vietnamese sentence and return as text format.
        
        Args:
            sentence: The Vietnamese sentence to tokenize
            
        Returns:
            Space-separated string with underscores for multi-word tokens
            
        Examples:
            >>> segmenter = VietnameseWordSegmenter()
            >>> segmenter.word_tokenize_as_text("Bác sĩ bây giờ có thể")
            "Bác_sĩ bây_giờ có_thể"
        """
        return self._segmenter.word_tokenize_as_text(sentence)


class VietnameseTokenizer:
    """
    Vietnamese tokenizer class.
    
    This class provides basic tokenization functionality for Vietnamese text.
    """
    
    def __init__(self, fixed_words: Optional[List[str]] = None):
        """
        Initialize the tokenizer.
        
        Args:
            fixed_words: Optional list of fixed words that should be treated as single tokens
        """
        if _parrotnlp is None:
            raise ImportError("VietnameseTokenizer requires the Rust extension to be built")
        
        if fixed_words:
            self._tokenizer = _parrotnlp.VietnameseTokenizerPy.with_fixed_words(fixed_words)
        else:
            self._tokenizer = _parrotnlp.VietnameseTokenizerPy()
    
    def tokenize(self, text: str) -> List[str]:
        """
        Tokenize Vietnamese text into basic tokens.
        
        Args:
            text: The Vietnamese text to tokenize
            
        Returns:
            List of tokens
        """
        return self._tokenizer.tokenize(text)
    
    def tokenize_with_tags(self, text: str, include_tags: bool = True) -> List[tuple]:
        """
        Tokenize Vietnamese text with token type tags.
        
        Args:
            text: The Vietnamese text to tokenize
            include_tags: Whether to include token type tags
            
        Returns:
            List of (token, tag) tuples if include_tags=True, list of tokens otherwise
        """
        return self._tokenizer.tokenize_with_tags(text, include_tags)
    
    def tokenize_as_text(self, text: str) -> str:
        """
        Tokenize Vietnamese text and return as space-separated string.
        
        Args:
            text: The Vietnamese text to tokenize
            
        Returns:
            Space-separated string of tokens
        """
        return self._tokenizer.tokenize_as_text(text)
