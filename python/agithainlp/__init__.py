"""AGI-ThaiNLP: Thai NLP functions rewritten from scratch without PyThaiNLP resources."""

__version__ = "0.1.0"

# Thai character constants
thai_consonants: str = "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ"  # 44 chars

thai_vowels: str = (
    "\u0e24\u0e26\u0e30\u0e31\u0e32\u0e33\u0e34\u0e35\u0e36\u0e37"
    + "\u0e38\u0e39\u0e40\u0e41\u0e42\u0e43\u0e44\u0e45\u0e4d\u0e47"
)  # 20
thai_lead_vowels: str = "\u0e40\u0e41\u0e42\u0e43\u0e44"  # เ แ โ ใ ไ (5)
thai_follow_vowels: str = "\u0e30\u0e32\u0e33\u0e45"  # ะ า ำ ๅ (4)
thai_above_vowels: str = "\u0e31\u0e34\u0e35\u0e36\u0e37\u0e4d\u0e47"  # ั ิ ี ึ ื ็ ็ (7)
thai_below_vowels: str = "\u0e38\u0e39"  # ุ ู (2)

thai_tonemarks: str = "\u0e48\u0e49\u0e4a\u0e4b"  # ่ ้ ๊ ๋ (4)

# Signs that can be part of a word
thai_signs: str = "\u0e2f\u0e3a\u0e46\u0e4c\u0e4d\u0e4e"  # ฯ ฺ ๆ ์ ็ ๎ (6)

# Any Thai character that can be part of a word
thai_letters: str = "".join(
    [thai_consonants, thai_vowels, thai_tonemarks, thai_signs]
)  # 74

# Section markers
thai_punctuations: str = "\u0e4f\u0e5a\u0e5b"  # ๏ ๚ ๛ (3)

thai_digits: str = "๐๑๒๓๔๕๖๗๘๙"  # 10
thai_symbols: str = "\u0e3f"  # ฿ Thai Baht

# All Thai characters in Unicode
thai_characters: str = "".join(
    [thai_letters, thai_punctuations, thai_digits, thai_symbols]
)

__all__ = [
    "thai_consonants",
    "thai_vowels",
    "thai_lead_vowels",
    "thai_follow_vowels",
    "thai_above_vowels",
    "thai_below_vowels",
    "thai_tonemarks",
    "thai_signs",
    "thai_letters",
    "thai_punctuations",
    "thai_digits",
    "thai_symbols",
    "thai_characters",
    # Tokenization
    "tcc_tokenize",
    "syllable_tokenize",
    "sent_tokenize",
    # Transliteration
    "romanize",
    # Soundex
    "soundex",
    "udom83",
    "lk82",
    # Utilities
    "collate",
    "num_to_thaiword",
    "thaiword_to_num",
    "thai_digit_to_arabic_digit",
    "arabic_digit_to_thai_digit",
    "remove_tonemark",
]

from agithainlp.tokenize import tcc_tokenize, syllable_tokenize, sent_tokenize
from agithainlp.transliterate import romanize
from agithainlp.soundex import soundex, udom83, lk82
from agithainlp.util import (
    collate,
    num_to_thaiword,
    thaiword_to_num,
    thai_digit_to_arabic_digit,
    arabic_digit_to_thai_digit,
    remove_tonemark,
)
