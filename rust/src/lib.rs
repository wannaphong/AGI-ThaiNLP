//! AGI-ThaiNLP: Thai NLP functions rewritten from scratch without PyThaiNLP resources.
//!
//! Provides:
//! - Thai character constants
//! - TCC (Thai Character Cluster) tokenization
//! - Sentence tokenization
//! - RTGS romanization
//! - Soundex (Udom83 and LK82)
//! - Thai collation
//! - Digit/number utilities

pub mod core;
pub mod tokenize;
pub mod transliterate;
pub mod soundex;
pub mod util;

pub use core::*;
pub use tokenize::{tcc_tokenize, syllable_tokenize, sent_tokenize};
pub use transliterate::romanize;
pub use soundex::{soundex, udom83, lk82};
pub use util::{collate, num_to_thaiword, thaiword_to_num, thai_digit_to_arabic_digit, arabic_digit_to_thai_digit, remove_tonemark};
