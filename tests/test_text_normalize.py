import pytest
import parrotnlp


def test_text_normalize():
    """Test text normalization function"""
    # Test basic text normalization
    test_text = "Tôi thích nghàn con mèo. Ð là ký tự đặc biệt."
    
    # Test with space tokenizer
    result_space = parrotnlp.text_normalize(test_text, "space")
    assert isinstance(result_space, str)
    assert "ngàn" in result_space  # Should normalize "nghàn" to "ngàn"
    assert "Đ" in result_space     # Should normalize "Ð" to "Đ"
    
    # Test with underthesea tokenizer
    result_underthesea = parrotnlp.text_normalize(test_text, "underthesea")
    assert isinstance(result_underthesea, str)
    assert "ngàn" in result_underthesea
    assert "Đ" in result_underthesea
    

def test_token_normalize():
    """Test token normalization function"""
    # Test character normalization enabled
    assert parrotnlp.token_normalize("nghàn", True) == "ngàn"
    assert parrotnlp.token_normalize("ð", True) == "đ"
    assert parrotnlp.token_normalize("Ð", True) == "Đ"
    
    # Test character normalization disabled
    assert parrotnlp.token_normalize("ð", False) == "ð"  # Should not change
    
    # Test tokens longer than 6 characters (should not be normalized)
    long_token = "thisstringislongerthan6characters"
    assert parrotnlp.token_normalize(long_token, True) == long_token
    
    # Test unknown token (should return as-is after character normalization)
    unknown_token = "xyz"
    assert parrotnlp.token_normalize(unknown_token, True) == unknown_token


def test_normalize_characters_in_text():
    """Test character normalization function"""
    # Test basic character mapping
    assert parrotnlp.normalize_characters_in_text("ð") == "đ"
    assert parrotnlp.normalize_characters_in_text("Ð") == "Đ"
    assert parrotnlp.normalize_characters_in_text("ðÐ") == "đĐ"
    
    # Test Unicode normalization (NFC)
    # This should handle combining characters
    test_with_combining = "a\u0300"  # a with combining grave accent
    result = parrotnlp.normalize_characters_in_text(test_with_combining)
    assert isinstance(result, str)
    
    # Test text without special characters
    normal_text = "Hello world"
    assert parrotnlp.normalize_characters_in_text(normal_text) == normal_text


def test_text_normalize_edge_cases():
    """Test edge cases for text normalization"""
    # Empty string
    assert parrotnlp.text_normalize("", "space") == ""
    
    # Single space
    assert parrotnlp.text_normalize(" ", "space") == ""
    
    # Multiple spaces
    result = parrotnlp.text_normalize("  a   b  ", "space")
    assert result == "a b"  # Should normalize to single spaces
    
    # Text with only special characters
    special_only = "ðÐ"
    result = parrotnlp.text_normalize(special_only, "space")
    assert "đ" in result and "Đ" in result


def test_integration():
    """Test integration between all normalization functions"""
    # Full pipeline test
    original = "Tôi thích nghàn ð mèo"
    
    # Should normalize both token ("nghàn" -> "ngàn") and character ("ð" -> "đ")
    normalized = parrotnlp.text_normalize(original, "space")
    
    assert "ngàn" in normalized
    assert "đ" in normalized
    assert "nghàn" not in normalized
    assert "ð" not in normalized


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
