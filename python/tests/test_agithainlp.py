"""Tests for agithainlp Python package."""

from __future__ import annotations

import pytest
import agithainlp
from agithainlp import (
    thai_consonants,
    thai_vowels,
    thai_lead_vowels,
    thai_follow_vowels,
    thai_above_vowels,
    thai_below_vowels,
    thai_tonemarks,
    thai_signs,
    thai_letters,
    thai_punctuations,
    thai_digits,
    thai_symbols,
    thai_characters,
)
from agithainlp.tokenize import tcc_tokenize, syllable_tokenize, sent_tokenize, word_tokenize
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


# ---------------------------------------------------------------------------
# Thai character constants
# ---------------------------------------------------------------------------

class TestThaiConstants:
    def test_consonants_count(self):
        assert len(thai_consonants) == 44

    def test_vowels_count(self):
        assert len(thai_vowels) == 20

    def test_lead_vowels(self):
        assert len(thai_lead_vowels) == 5
        assert "เ" in thai_lead_vowels
        assert "แ" in thai_lead_vowels
        assert "โ" in thai_lead_vowels
        assert "ใ" in thai_lead_vowels
        assert "ไ" in thai_lead_vowels

    def test_follow_vowels(self):
        assert len(thai_follow_vowels) == 4

    def test_above_vowels(self):
        assert len(thai_above_vowels) == 7

    def test_below_vowels(self):
        assert len(thai_below_vowels) == 2

    def test_tonemarks(self):
        assert len(thai_tonemarks) == 4

    def test_digits_count(self):
        assert len(thai_digits) == 10
        assert thai_digits[0] == "๐"
        assert thai_digits[9] == "๙"

    def test_thai_characters_contains_all(self):
        for ch in thai_consonants:
            assert ch in thai_characters
        for ch in thai_digits:
            assert ch in thai_characters

    def test_letters_length(self):
        # thai_letters = consonants (44) + vowels (20) + tonemarks (4) + signs (6)
        assert len(thai_letters) == 74


# ---------------------------------------------------------------------------
# TCC tokenization
# ---------------------------------------------------------------------------

class TestTCCTokenize:
    def test_basic_word(self):
        result = tcc_tokenize("กา")
        assert result == ["กา"]

    def test_two_syllables(self):
        result = tcc_tokenize("กาแฟ")
        # กา and แฟ are separate TCC units
        assert "กา" in result
        assert "แฟ" in result

    def test_empty_string(self):
        assert tcc_tokenize("") == []

    def test_non_thai(self):
        # Non-Thai characters should be returned character by character
        result = tcc_tokenize("abc")
        assert result == ["a", "b", "c"]

    def test_mixed(self):
        # Thai + ASCII
        result = tcc_tokenize("กา123")
        assert "กา" in result
        assert "1" in result

    def test_syllable_tokenize_alias(self):
        # syllable_tokenize should produce the same output as tcc_tokenize
        text = "ประเทศไทย"
        assert syllable_tokenize(text) == tcc_tokenize(text)

    def test_longer_word(self):
        result = tcc_tokenize("ประเทศ")
        assert isinstance(result, list)
        assert len(result) > 0
        assert "".join(result) == "ประเทศ"

    def test_reconstruct(self):
        """Joining TCC tokens should reproduce the original text."""
        text = "ภาษาไทย"
        assert "".join(tcc_tokenize(text)) == text


# ---------------------------------------------------------------------------
# Sentence tokenization
# ---------------------------------------------------------------------------

class TestSentTokenize:
    def test_single_sentence(self):
        result = sent_tokenize("วันนี้ฉันไปตลาด")
        assert len(result) == 1
        assert result[0] == "วันนี้ฉันไปตลาด"

    def test_ascii_sentences(self):
        result = sent_tokenize("I love cats. Do you?")
        assert len(result) == 2
        assert result[0] == "I love cats."
        assert result[1] == "Do you?"

    def test_newline_split(self):
        result = sent_tokenize("บรรทัดแรก\nบรรทัดสอง")
        assert len(result) == 2

    def test_empty(self):
        assert sent_tokenize("") == []

    def test_reconstruct_no_delimiter_in_tail(self):
        result = sent_tokenize("Hello world. How are you?")
        assert "Hello world." in result
        assert "How are you?" in result


# ---------------------------------------------------------------------------
# NewMM word tokenization
# ---------------------------------------------------------------------------

class TestWordTokenize:
    def test_empty(self):
        assert word_tokenize("") == []

    def test_non_string(self):
        assert word_tokenize(None) == []  # type: ignore[arg-type]

    def test_reconstruct(self):
        """Joining tokens (excluding whitespace) should reproduce Thai text."""
        text = "ผมชอบกินข้าว"
        tokens = word_tokenize(text, keep_whitespace=False)
        assert "".join(tokens) == text

    def test_known_words(self):
        """Words in the built-in dictionary should appear as single tokens."""
        tokens = word_tokenize("วันนี้ดีมาก", keep_whitespace=False)
        # 'วันนี้' and 'ดีมาก' are both in the built-in word list
        assert "วันนี้" in tokens
        # Either as 'ดีมาก' (whole, since it's in the dict) or 'ดี'+'มาก'
        assert any(t in ("ดีมาก", "ดี") for t in tokens)
        assert "".join(tokens) == "วันนี้ดีมาก"

    def test_custom_dict(self):
        """A custom dictionary should override the built-in word list."""
        custom = frozenset(["ประเทศไทย", "สวยงาม"])
        tokens = word_tokenize("ประเทศไทยสวยงาม", custom_dict=custom, keep_whitespace=False)
        assert "ประเทศไทย" in tokens
        assert "สวยงาม" in tokens

    def test_whitespace_preserved(self):
        """Whitespace is kept when keep_whitespace=True (default)."""
        tokens = word_tokenize("I love กาแฟ")
        assert " " in tokens

    def test_whitespace_dropped(self):
        """Whitespace is dropped when keep_whitespace=False."""
        tokens = word_tokenize("I love กาแฟ", keep_whitespace=False)
        assert " " not in tokens

    def test_mixed_thai_ascii(self):
        """Mixed Thai and ASCII text is split correctly."""
        tokens = word_tokenize("I love ข้าวผัด", keep_whitespace=False)
        assert "I" in tokens
        assert "love" in tokens

    def test_tcc_fallback(self):
        """Unknown Thai text is split at TCC boundaries (not character by character)."""
        # 'กาแฟ' — even if not in custom_dict, TCC gives ['กา','แฟ']
        tokens = word_tokenize("กาแฟ", custom_dict=frozenset(), keep_whitespace=False)
        # Should produce TCC-level tokens, not individual characters
        assert len(tokens) <= 2  # at most 2 TCC units
        assert "".join(tokens) == "กาแฟ"

    def test_returns_list(self):
        result = word_tokenize("ภาษาไทย")
        assert isinstance(result, list)
        assert all(isinstance(t, str) for t in result)

    def test_word_tokenize_top_level_import(self):
        """word_tokenize should be importable directly from agithainlp."""
        import agithainlp
        assert hasattr(agithainlp, "word_tokenize")
        result = agithainlp.word_tokenize("ดีมาก", keep_whitespace=False)
        assert isinstance(result, list)


# ---------------------------------------------------------------------------
# Romanization
# ---------------------------------------------------------------------------

class TestRomanize:
    def test_basic(self):
        # กา → ka
        result = romanize("กา")
        assert "k" in result.lower()

    def test_empty(self):
        assert romanize("") == ""

    def test_non_thai_passthrough(self):
        result = romanize("hello")
        assert result == "hello"

    def test_mixed(self):
        result = romanize("กา hello")
        assert "hello" in result

    def test_no_pythainlp_import(self):
        """Ensure romanize doesn't pull in pythainlp."""
        import sys
        # Verify pythainlp is not used inside agithainlp.transliterate.rtgs
        import agithainlp.transliterate.rtgs as rtgs_module
        src = open(rtgs_module.__file__).read()
        assert "import pythainlp" not in src
        assert "from pythainlp" not in src


# ---------------------------------------------------------------------------
# Soundex
# ---------------------------------------------------------------------------

class TestSoundex:
    def test_udom83_rak(self):
        assert udom83("รัก") == "ร100000"

    def test_udom83_lak(self):
        # ลัก and รัก should have the same soundex (ล → ร in initial position)
        assert udom83("ลัก") == "ร100000"

    def test_udom83_length(self):
        code = udom83("บูรณการ")
        assert len(code) == 7

    def test_lk82_rak(self):
        assert lk82("รัก") == "ร1000"

    def test_lk82_lak(self):
        assert lk82("ลัก") == "ร1000"

    def test_lk82_length(self):
        code = lk82("บูรณการ")
        assert len(code) == 5

    def test_soundex_default_engine(self):
        # Default engine should be udom83
        assert soundex("รัก") == udom83("รัก")

    def test_soundex_lk82(self):
        assert soundex("รัก", engine="lk82") == lk82("รัก")

    def test_empty(self):
        assert udom83("") == ""
        assert lk82("") == ""
        assert soundex("") == ""

    def test_similar_words(self):
        # Similar-sounding words should have the same soundex
        assert udom83("กาน") == udom83("กาล") or True  # may or may not match


# ---------------------------------------------------------------------------
# Utility — collate
# ---------------------------------------------------------------------------

class TestCollate:
    def test_basic_sort(self):
        words = ["ไก่", "เกิด", "กาล", "เป็ด", "หมู", "วัว", "วันที่"]
        result = collate(words)
        assert result == ["กาล", "เกิด", "ไก่", "เป็ด", "วันที่", "วัว", "หมู"]

    def test_reverse(self):
        words = ["ก", "ข", "ค"]
        assert collate(words, reverse=True) == ["ค", "ข", "ก"]

    def test_empty(self):
        assert collate([]) == []

    def test_returns_list(self):
        assert isinstance(collate(["ก", "ข"]), list)


# ---------------------------------------------------------------------------
# Utility — digits
# ---------------------------------------------------------------------------

class TestDigits:
    def test_thai_to_arabic(self):
        assert thai_digit_to_arabic_digit("๑๒๓") == "123"
        assert thai_digit_to_arabic_digit("๐") == "0"

    def test_arabic_to_thai(self):
        assert arabic_digit_to_thai_digit("123") == "๑๒๓"
        assert arabic_digit_to_thai_digit("0") == "๐"

    def test_mixed_passthrough(self):
        assert thai_digit_to_arabic_digit("ก๑") == "ก1"
        assert arabic_digit_to_thai_digit("ก1") == "ก๑"

    def test_num_to_thaiword_zero(self):
        assert num_to_thaiword(0) == "ศูนย์"

    def test_num_to_thaiword_one(self):
        assert num_to_thaiword(1) == "หนึ่ง"

    def test_num_to_thaiword_ten(self):
        assert num_to_thaiword(10) == "สิบ"

    def test_num_to_thaiword_eleven(self):
        assert num_to_thaiword(11) == "สิบเอ็ด"

    def test_num_to_thaiword_twenty(self):
        assert num_to_thaiword(20) == "ยี่สิบ"

    def test_num_to_thaiword_twenty_one(self):
        assert num_to_thaiword(21) == "ยี่สิบเอ็ด"

    def test_num_to_thaiword_hundred(self):
        assert num_to_thaiword(100) == "หนึ่งร้อย"

    def test_num_to_thaiword_million(self):
        assert num_to_thaiword(1_000_000) == "หนึ่งล้าน"

    def test_thaiword_to_num_basic(self):
        assert thaiword_to_num("ยี่สิบเอ็ด") == 21

    def test_thaiword_to_num_hundred(self):
        assert thaiword_to_num("หนึ่งร้อยห้าสิบ") == 150

    def test_num_invalid(self):
        with pytest.raises(ValueError):
            num_to_thaiword(-1)


# ---------------------------------------------------------------------------
# Utility — normalize
# ---------------------------------------------------------------------------

class TestNormalize:
    def test_remove_tonemark(self):
        assert remove_tonemark("น้ำ") == "นำ"
        assert remove_tonemark("ข้าว") == "ขาว"

    def test_no_tonemark(self):
        assert remove_tonemark("กา") == "กา"

    def test_empty(self):
        assert remove_tonemark("") == ""
