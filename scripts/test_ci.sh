#!/bin/bash
# Test script for CI/CD workflows

set -e

echo "=== Testing CI/CD Workflows Locally ==="

# Test 1: Check project structure
echo "1. Checking project structure..."
ls -la
ls -la parrotnlp/
ls -la src/
ls -la tests/

# Test 2: Validate configuration files
echo "2. Validating configuration files..."
python -c "import tomllib; tomllib.load(open('pyproject.toml', 'rb')); print('✅ pyproject.toml is valid')"
cargo check --message-format=short

# Test 4: Check for encoding references
echo "4. Checking for encoding references..."
FOUND_ENCODING=$(find . -type f \( -name "*.py" -o -name "*.rs" -o -name "*.toml" -o -name "*.md" \) \
    -not -path "./.git/*" \
    -not -path "./target/*" \
    -not -path "./dist/*" \
    -not -path "./__pycache__/*" \
    -not -path "./.github/*" \
    -not -name "test_ci.sh" \
    -exec grep -l "get_encoding\|list_encoding_names\|encoding_for_model" {} \; 2>/dev/null || true)

if [ -n "$FOUND_ENCODING" ]; then
    echo "❌ Found encoding references in:"
    echo "$FOUND_ENCODING"
    exit 1
else
    echo "✅ No encoding references found"
fi

# Test 5: Build and test package
echo "5. Building and testing package..."
maturin develop --release
python tests/test_word_tokenize.py

# Test 6: Verify functionality
echo "6. Verifying functionality..."
python -c "
import parrotnlp
print('✅ parrotnlp imports successfully')

# Check available functions
available = [f for f in dir(parrotnlp) if not f.startswith('_')]
expected = ['VietnameseTokenizer', 'VietnameseWordSegmenter', 'word_tokenize']

for func in expected:
    if func in available:
        print(f'✅ {func} is available')
    else:
        print(f'❌ {func} is missing')
        exit(1)

# Test basic functionality
try:
    result = parrotnlp.word_tokenize('Chàng trai 9X Quảng Trị')
    print(f'✅ word_tokenize works: {result}')
except Exception as e:
    print(f'❌ word_tokenize failed: {e}')
    exit(1)
"

echo "=== All tests passed! ==="
