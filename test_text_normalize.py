#!/usr/bin/env python3
"""
Test script for text normalization functionality
"""
import parrotnlp

def test_text_normalization():
    print("Testing text normalization...")
    
    # Test text normalization
    test_text = "Tôi thích nghàn con mèo. Ð là ký tự đặc biệt."
    print(f"Original text: {test_text}")
    
    # Test with underthesea tokenizer
    normalized_underthesea = parrotnlp.text_normalize(test_text, "underthesea")
    print(f"Normalized (underthesea): {normalized_underthesea}")
    
    # Test with space tokenizer
    normalized_space = parrotnlp.text_normalize(test_text, "space")
    print(f"Normalized (space): {normalized_space}")
    
    # Test token normalization
    print("\nTesting token normalization...")
    test_tokens = ["nghàn", "nghành", "nghã", "ð", "Ð"]
    for token in test_tokens:
        normalized_token = parrotnlp.token_normalize(token, True)
        print(f"'{token}' -> '{normalized_token}'")
    
    # Test character normalization
    print("\nTesting character normalization...")
    test_chars = "ðÐ"
    normalized_chars = parrotnlp.normalize_characters_in_text(test_chars)
    print(f"'{test_chars}' -> '{normalized_chars}'")
    
    print("All tests completed!")

if __name__ == "__main__":
    test_text_normalization()
