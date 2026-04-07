"""Royal Thai General System of Transcription (RTGS) romanization.

This is a reimplementation without using PyThaiNLP resources.
Uses TCC tokenization for word segmentation instead of PyThaiNLP's word_tokenize.

References:
    * Royal Institute of Thailand, RTGS:
      https://en.wikipedia.org/wiki/Royal_Thai_General_System_of_Transcription
"""

from __future__ import annotations

import re

# Thai consonants string (copied from core module to avoid circular import)
_THAI_CONSONANTS: str = "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ"

# Romanized vowels for checking
_ROMANIZED_VOWELS: str = "aeiou"

# Vowel pattern substitutions: (Thai pattern, romanized replacement)
# Patterns use * as placeholder for Thai consonant class [ก-ฮ]
_VOWEL_PATTERNS_RAW: str = r"""เ*ียว,\1iao
แ*็ว,\1aeo
เ*ือย,\1ueai
แ*ว,\1aeo
เ*็ว,\1eo
เ*ว,\1eo
*ิว,\1io
*วย,\1uai
เ*ย,\1oei
*อย,\1oi
โ*ย,\1oi
*ุย,\1ui
*าย,\1ai
ไ*ย,\1ai
*ัย,\1ai
ไ**,\1\2ai
ไ*,\1ai
ใ*,\1ai
*ว*,\1ua\2
*ัวะ,\1ua
*ัว,\1ua
เ*ือะ,\1uea
เ*ือ,\1uea
เ*ียะ,\1ia
เ*ีย,\1ia
เ*อะ,\1oe
เ*อ,\1oe
เ*ิ,\1oe
*อ,\1o
เ*าะ,\1o
เ*็,\1e
โ*ะ,\1o
โ*,\1o
แ*ะ,\1ae
แ*,\1ae
เ*าะ,\1e
*าว,\1ao
เ*า,\1ao
เ*,\1e
*ู,\1u
*ุ,\1u
*ื,\1ue
*ึ,\1ue
*ี,\1i
*ิ,\1i
*ำ,\1am
*า,\1a
*ั,\1a
*ะ,\1a
#ฤ,\1rue
$ฤ,\1ri"""

_C = f"([{_THAI_CONSONANTS}])"
_VOWEL_PATTERNS_RAW = _VOWEL_PATTERNS_RAW.replace("*", _C)
_VOWEL_PATTERNS_RAW = _VOWEL_PATTERNS_RAW.replace("#", "([คนพมห])")
_VOWEL_PATTERNS_RAW = _VOWEL_PATTERNS_RAW.replace("$", "([กตทปศส])")

_VOWELS: list[list[str]] = [
    line.split(",") for line in _VOWEL_PATTERNS_RAW.split("\n")
]

# Initial and final consonant romanizations: {Thai: [initial, final]}
_CONSONANTS: dict[str, list[str]] = {
    "ก": ["k", "k"],
    "ข": ["kh", "k"],
    "ฃ": ["kh", "k"],
    "ค": ["kh", "k"],
    "ฅ": ["kh", "k"],
    "ฆ": ["kh", "k"],
    "ง": ["ng", "ng"],
    "จ": ["ch", "t"],
    "ฉ": ["ch", "t"],
    "ช": ["ch", "t"],
    "ซ": ["s", "t"],
    "ฌ": ["ch", "t"],
    "ญ": ["y", "n"],
    "ฎ": ["d", "t"],
    "ฏ": ["t", "t"],
    "ฐ": ["th", "t"],
    "ฑ": ["th", "t"],
    "ฒ": ["th", "t"],
    "ณ": ["n", "n"],
    "ด": ["d", "t"],
    "ต": ["t", "t"],
    "ถ": ["th", "t"],
    "ท": ["th", "t"],
    "ธ": ["th", "t"],
    "น": ["n", "n"],
    "บ": ["b", "p"],
    "ป": ["p", "p"],
    "ผ": ["ph", "p"],
    "ฝ": ["f", "p"],
    "พ": ["ph", "p"],
    "ฟ": ["f", "p"],
    "ภ": ["ph", "p"],
    "ม": ["m", "m"],
    "ย": ["y", ""],
    "ร": ["r", "n"],
    "ฤ": ["rue", ""],
    "ล": ["l", "n"],
    "ว": ["w", ""],
    "ศ": ["s", "t"],
    "ษ": ["s", "t"],
    "ส": ["s", "t"],
    "ห": ["h", ""],
    "ฬ": ["l", "n"],
    "อ": ["", ""],
    "ฮ": ["h", ""],
}

_THANTHAKHAT: str = "\u0e4c"  # ์ (kills pronunciation)
_RE_CONSONANT: re.Pattern[str] = re.compile(f"[{_THAI_CONSONANTS}]")
_RE_NORMALIZE: re.Pattern[str] = re.compile(
    f"จน์|มณ์|ณฑ์|ทร์|ตร์|[{_THAI_CONSONANTS}]{_THANTHAKHAT}|"
    f"[{_THAI_CONSONANTS}][\u0e30-\u0e39]{_THANTHAKHAT}"
    r"|[\u0e2f\u0e46\u0e48-\u0e4f\u0e5a\u0e5b]"  # signs/tonemarks/punctuation
)


def _normalize(word: str) -> str:
    """Remove silenced consonants (การันต์), tonemarks, and other silent signs."""
    return _RE_NORMALIZE.sub("", word)


def _replace_vowels(word: str) -> str:
    """Replace Thai vowel patterns with romanized equivalents."""
    for pattern, replacement in _VOWELS:
        word = re.sub(pattern, replacement, word)
    return word


def _replace_consonants(word: str, consonants: str) -> str:
    """Replace remaining Thai consonants with their romanized initials/finals."""
    _HO_HIP = "\u0e2b"  # ห
    _RO_RUA = "\u0e23"  # ร
    _LO_LING = "\u0e25"  # ล
    _WO_WAEN = "\u0e27"  # ว
    _DOUBLE_RO_RUA = _RO_RUA + _RO_RUA

    _CLUSTER_SECOND = {_RO_RUA, _LO_LING, _WO_WAEN}

    if not consonants:
        return word

    skip = False
    mod_chars: list[str] = []
    j = 0
    vowel_seen = False

    for i, ch in enumerate(word):
        if skip:
            skip = False
            j += 1
            continue
        if ch not in _CONSONANTS:
            vowel_seen = True
            mod_chars.append(ch)
        elif len(mod_chars) == 0 and ch == _HO_HIP and len(consonants) != 1:
            j += 1
        elif word[i:] == _DOUBLE_RO_RUA:
            skip = True
            mod_chars.append("an")
            vowel_seen = True
            j += 1
        elif word[i : i + 2] == _DOUBLE_RO_RUA:
            skip = True
            mod_chars.append("a")
            vowel_seen = True
            j += 1
        elif not vowel_seen:
            has_initial = any(c and c not in _ROMANIZED_VOWELS for c in mod_chars)
            if not has_initial:
                initial = _CONSONANTS[consonants[j]][0]
                if initial:
                    mod_chars.append(initial)
                j += 1
            else:
                is_cluster = ch in _CLUSTER_SECOND
                is_last = i + 1 >= len(word)
                next_is_vowel = not is_last and word[i + 1] not in _CONSONANTS

                if is_cluster and (next_is_vowel or not is_last):
                    mod_chars.append(_CONSONANTS[consonants[j]][0])
                    j += 1
                elif not is_cluster and not is_last:
                    mod_chars.append("a")
                    initial = _CONSONANTS[consonants[j]][0]
                    if initial:
                        mod_chars.append(initial)
                    vowel_seen = False
                    j += 1
                elif next_is_vowel:
                    mod_chars.append(_CONSONANTS[consonants[j]][0])
                    j += 1
                elif is_last:
                    mod_chars.append("o")
                    mod_chars.append(_CONSONANTS[consonants[j]][1])
                    vowel_seen = True
                    j += 1
                else:
                    mod_chars.append("o")
                    mod_chars.append(_CONSONANTS[consonants[j]][1])
                    vowel_seen = True
                    j += 1
        else:
            next_is_vowel = i + 1 < len(word) and word[i + 1] not in _CONSONANTS
            if next_is_vowel:
                mod_chars.append(_CONSONANTS[consonants[j]][0])
                vowel_seen = False
                j += 1
            else:
                mod_chars.append(_CONSONANTS[consonants[j]][1])
                j += 1

    return "".join(mod_chars)


def _romanize_word(word: str) -> str:
    """Romanize a single Thai word or token using RTGS rules."""
    if word == "ห":
        return ""

    normalized = _normalize(word)
    vowels_replaced = _replace_vowels(normalized)
    consonants = _RE_CONSONANT.findall(vowels_replaced)

    # Two-character all-consonant words: insert implicit 'o' vowel
    if len(vowels_replaced) == 2 and len(consonants) == 2:
        vowels_replaced = vowels_replaced[0] + "o" + vowels_replaced[1]

    result = _replace_consonants(vowels_replaced, "".join(consonants))
    return result


def romanize(text: str) -> str:
    """Render Thai text in Latin alphabet using RTGS.

    Royal Thai General System of Transcription (RTGS) is the official
    system published by the Royal Institute of Thailand.

    This implementation uses TCC-based syllable tokenization instead of
    dictionary-based word tokenization, so it works without any external
    language resources.

    :param str text: Thai text to romanize
    :return: romanized text (Latin alphabet)
    :rtype: str

    :Example:

        >>> romanize("กาแฟ")
        'kafae'
        >>> romanize("ประเทศไทย")
        'prathet thai'
        >>> romanize("สวัสดี")
        'sawatdi'
    """
    if not text or not isinstance(text, str):
        return ""

    # Import here to avoid circular imports
    from agithainlp.tokenize.tcc import tcc_tokenize

    parts: list[str] = []
    for ch in text:
        if "\u0e00" <= ch <= "\u0e7f":
            # Thai character — collect for romanization
            parts.append(ch)
        else:
            # Non-Thai (spaces, ASCII, etc.) — pass through
            parts.append(ch)

    # Romanize each character group by splitting on non-Thai runs
    result: list[str] = []
    i = 0
    while i < len(text):
        ch = text[i]
        if "\u0e00" <= ch <= "\u0e7f":
            # Collect Thai run
            j = i
            while j < len(text) and "\u0e00" <= text[j] <= "\u0e7f":
                j += 1
            thai_run = text[i:j]
            # Tokenize the Thai run into TCC units and romanize each
            tokens = tcc_tokenize(thai_run)
            romanized_tokens = [_romanize_word(t) for t in tokens]
            result.append("".join(romanized_tokens))
            i = j
        else:
            result.append(ch)
            i += 1

    return "".join(result)
