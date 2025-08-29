use pyo3::{
    prelude::*,
};

use crate::word_tokenize::{VietnameseWordSegmenter, VietnameseTokenizer};


#[pyclass]
struct VietnameseWordSegmenterPy {
    segmenter: VietnameseWordSegmenter,
}

#[pymethods]
impl VietnameseWordSegmenterPy {
    #[new]
    fn new() -> Self {
        Self {
            segmenter: VietnameseWordSegmenter::new(),
        }
    }

    #[staticmethod]
    #[pyo3(name = "with_fixed_words")]
    fn with_fixed_words(fixed_words: Vec<String>) -> Self {
        Self {
            segmenter: VietnameseWordSegmenter::with_fixed_words(&fixed_words),
        }
    }

    fn word_tokenize(&self, sentence: &str) -> Vec<String> {
        self.segmenter.word_tokenize(sentence)
    }

    fn word_tokenize_as_text(&self, sentence: &str) -> String {
        self.segmenter.word_tokenize_as_text(sentence)
    }

    fn word_tokenize_with_options(
        &self,
        sentence: &str,
        format_as_text: bool,
        use_token_normalize: bool,
        fixed_words: Vec<String>,
    ) -> Vec<String> {
        self.segmenter.word_tokenize_with_options(
            sentence,
            format_as_text,
            use_token_normalize,
            &fixed_words,
        )
    }
}

#[pyclass]
struct VietnameseTokenizerPy {
    tokenizer: VietnameseTokenizer,
}

#[pymethods]
impl VietnameseTokenizerPy {
    #[new]
    fn new() -> Self {
        Self {
            tokenizer: VietnameseTokenizer::new(),
        }
    }

    #[staticmethod]
    #[pyo3(name = "with_fixed_words")]
    fn with_fixed_words(fixed_words: Vec<String>) -> Self {
        Self {
            tokenizer: VietnameseTokenizer::with_fixed_words(&fixed_words),
        }
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        self.tokenizer.tokenize(text)
    }

    fn tokenize_with_tags(&self, text: &str, include_tags: bool) -> Vec<(String, String)> {
        let tokens = self.tokenizer.tokenize_with_tags(text, include_tags);
        tokens
            .into_iter()
            .map(|token| (token.text, format!("{:?}", token.token_type)))
            .collect()
    }

    fn tokenize_as_text(&self, text: &str) -> String {
        self.tokenizer.tokenize_as_text(text)
    }
}

#[pymodule]
fn _parrotnlp(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<VietnameseWordSegmenterPy>()?;
    m.add_class::<VietnameseTokenizerPy>()?;
    Ok(())
}
