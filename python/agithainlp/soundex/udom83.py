"""Thai soundex — Udom83 system.

Original paper:
    Wannee Udompanich. String searching for Thai alphabet using Soundex
    compression technique. Master Thesis of Department of Computer
    Engineering Graduate School, Chulalongkorn University, 1983.
    https://cuir.car.chula.ac.th/handle/123456789/48471

Python implementation based on:
    Korakot Chaovavanich
    https://gist.github.com/korakot/0b772e09340cac2f493868da035597e8

Reimplemented without using PyThaiNLP resources.
"""

from __future__ import annotations

import re

# Thai consonants string (avoid circular import)
_THAI_CONSONANTS: str = "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ"

_THANTHAKHAT: str = "\u0e4c"

# Preprocessing regex patterns
_RE_1: re.Pattern[str] = re.compile(r"รร([\u0e40-\u0e44])")  # รร + lead vowel
_RE_2: re.Pattern[str] = re.compile(
    f"รร([{_THAI_CONSONANTS}][{_THAI_CONSONANTS}\u0e40-\u0e44])"
)
_RE_3: re.Pattern[str] = re.compile(
    f"รร([{_THAI_CONSONANTS}][\u0e30-\u0e39\u0e48-\u0e4c])"
)
_RE_4: re.Pattern[str] = re.compile(r"รร")
_RE_5: re.Pattern[str] = re.compile(f"ไ([{_THAI_CONSONANTS}]ย)")
_RE_6: re.Pattern[str] = re.compile(f"[ไใ]([{_THAI_CONSONANTS}])")
_RE_7: re.Pattern[str] = re.compile(r"\u0e33(ม[\u0e30-\u0e39])")  # ำ + ม + vowel
_RE_8: re.Pattern[str] = re.compile(r"\u0e33ม")   # ำม
_RE_9: re.Pattern[str] = re.compile(r"\u0e33")    # ำ → ม
_RE_10: re.Pattern[str] = re.compile(
    f"จน์|มณ์|ณฑ์|ทร์|ตร์|"
    f"[{_THAI_CONSONANTS}]{_THANTHAKHAT}|[{_THAI_CONSONANTS}]"
    f"[\u0e30-\u0e39]{_THANTHAKHAT}"
)
_RE_11: re.Pattern[str] = re.compile(r"[\u0e30-\u0e4c]")  # vowel diacritics

# Translation table 1: normalize initial consonants
_TRANS1: dict[int, int] = str.maketrans(
    "กขฃคฅฆงจฉชฌซศษสฎดฏตฐฑฒถทธณนบปผพภฝฟมญยรลฬฤฦวอหฮ",
    "กขขขขขงจชชชสสสสดดตตททททททนนบปพพพฟฟมยยรรรรรวอฮฮ",
)
# Translation table 2: encode consonants to digits
_TRANS2: dict[int, int] = str.maketrans(
    "มวำกขฃคฅฆงยญณนฎฏดตศษสบปพภผฝฟหอฮจฉชซฌฐฑฒถทธรฤลฦ",
    "0001111112233344444445555666666777778888889999",
)


def udom83(text: str) -> str:
    """Convert Thai text to phonetic code using the Udom83 soundex system.

    :param str text: Thai word
    :return: Udom83 soundex code (7 characters)
    :rtype: str

    :Example:

        >>> udom83("ลัก")
        'ร100000'
        >>> udom83("รัก")
        'ร100000'
        >>> udom83("บูรณการ")
        'บ931900'
        >>> udom83("ปัจจุบัน")
        'ป775300'
    """
    if not text or not isinstance(text, str):
        return ""

    text = _RE_1.sub(r"ัน\1", text)
    text = _RE_2.sub(r"ั\1", text)
    text = _RE_3.sub(r"ัน\1", text)
    text = _RE_4.sub("ัน", text)
    text = _RE_5.sub(r"\1", text)
    text = _RE_6.sub(r"\1ย", text)
    text = _RE_7.sub(r"ม\1", text)
    text = _RE_8.sub("ม", text)
    text = _RE_9.sub("ม", text)
    text = _RE_10.sub("", text)
    text = _RE_11.sub("", text)

    if not text:
        return ""

    code = "".join(
        [text[0].translate(_TRANS1), text[1:].translate(_TRANS2), "000000"]
    )
    return code[:7]
