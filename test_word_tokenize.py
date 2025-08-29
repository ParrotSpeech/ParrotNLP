#!/usr/bin/env python3
"""
Test script for word_tokenize functionality
"""

import parrotnlp
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__)))


def test_word_tokenize():
    print("=== Testing word_tokenize functionality ===\n")

    # Test 1: Basic word tokenization
    print("1. Basic word tokenization:")
    text = "Chàng trai 9X Quảng Trị khởi nghiệp từ nấm sò"
    result = parrotnlp.word_tokenize(text)
    print(f"Input: {text}")
    print(f"Output: {result}")
    print()

    # Test 2: Text format
    print("2. Text format:")
    result = parrotnlp.word_tokenize(text, format="text")
    print(f"Input: {text}")
    print(f"Output: {result}")
    print()

    # Test 3: With fixed words
    print("3. With fixed words:")
    text2 = "Viện Nghiên Cứu chiến lược quốc gia về học máy"
    fixed_words = ["Viện Nghiên Cứu", "học máy"]
    result = parrotnlp.word_tokenize(text2, fixed_words=fixed_words)
    print(f"Input: {text2}")
    print(f"Fixed words: {fixed_words}")
    print(f"Output: {result}")
    print()

    # Test 4: Class-based API
    print("4. Class-based API:")
    from parrotnlp import VietnameseWordSegmenter
    segmenter = VietnameseWordSegmenter()
    text3 = "Bác sĩ bây giờ có thể thản nhiên báo tin bệnh nhân bị ung thư"
    result = segmenter.word_tokenize(text3)
    print(f"Input: {text3}")
    print(f"Output: {result}")
    print()

    # Test 5: Text format with class
    print("5. Text format with class:")
    result = segmenter.word_tokenize_as_text(text3)
    print(f"Input: {text3}")
    print(f"Output: {result}")
    print()

    # Test 6: With fixed words using class
    print("6. With fixed words using class:")
    segmenter_fixed = VietnameseWordSegmenter(
        fixed_words=["bác sĩ", "bệnh nhân"])
    result = segmenter_fixed.word_tokenize(text3)
    print(f"Input: {text3}")
    print(f"Fixed words: ['bác sĩ', 'bệnh nhân']")
    print(f"Output: {result}")
    print()

    print("=== All tests completed successfully! ===")


if __name__ == "__main__":
    test_word_tokenize()
