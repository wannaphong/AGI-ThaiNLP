"""Thai soundex — LK82 system.

Original paper:
    Vichit Lorchirachoonkul. 1982. A Thai soundex system.
    Information Processing & Management, 18(5):243–255.
    https://doi.org/10.1016/0306-4573(82)90003-6

Python implementation based on:
    Korakot Chaovavanich
    https://gist.github.com/korakot/0b772e09340cac2f493868da035597e8

Reimplemented without using PyThaiNLP resources.
"""

from __future__ import annotations

import re

# Translation table 1: normalize initial consonants
_TRANS1: dict[int, int] = str.maketrans(
    "กขฃคฅฆงจฉชฌซศษสญยฎดฏตณนฐฑฒถทธบปผพภฝฟมรลฬฤฦวหฮอ",
    "กกกกกกงจชชชซซซซยยดดตตนนททททททบปพพพฟฟมรรรรรวหหอ",
)
# Translation table 2: encode consonants/vowels to characters
_TRANS2: dict[int, int] = str.maketrans(
    "กขฃคฅฆงจฉชซฌฎฏฐฑฒดตถทธศษสญณนรลฬฤฦบปพฟภผฝมำยวไใหฮาๅึืเแโุูอ",
    "1111112333333333333333333444444445555555667777889AAABCDEEF",
)

# Remove silenced consonants (การันต์)
_RE_KARANT: re.Pattern[str] = re.compile(
    r"จน์|มณ์|ณฑ์|ทร์|ตร์|[ก-ฮ]์|[ก-ฮ][ะ-ู]์"
)
# Remove signs with no phonetic value
_RE_SIGN: re.Pattern[str] = re.compile(r"[\u0e2f\u0e3a\u0e46\u0e47\u0e4d]")
# Tonemark characters
_RE_TONE: re.Pattern[str] = re.compile(r"[\u0e48\u0e49\u0e4a\u0e4b]")


def lk82(text: str) -> str:
    """Convert Thai text to phonetic code using the LK82 soundex system.

    :param str text: Thai word
    :return: LK82 soundex code (5 characters)
    :rtype: str

    :Example:

        >>> lk82("ลัก")
        'ร1000'
        >>> lk82("รัก")
        'ร1000'
        >>> lk82("บูรณการ")
        'บE419'
        >>> lk82("ปัจจุบัน")
        'ป3E54'
    """
    if not text or not isinstance(text, str):
        return ""

    text = _RE_TONE.sub("", text)    # remove tone marks
    text = _RE_KARANT.sub("", text)  # remove silenced consonants
    text = _RE_SIGN.sub("", text)    # remove silent signs

    if not text:
        return ""

    res: list[str] = []

    # Encode the first character
    if "ก" <= text[0] <= "ฮ":
        res.append(text[0].translate(_TRANS1))
        text = text[1:]
    else:
        if len(text) > 1:
            res.append(text[1].translate(_TRANS1))
        res.append(text[0].translate(_TRANS2))
        text = text[2:]

    # Encode remaining characters
    i_v: int | None = None  # index of last vowel separator
    n = len(text)
    for i, ch in enumerate(text):
        if ch in "\u0e30\u0e31\u0e34\u0e35":
            # Sara A, Mai Han-Akat, Sara I, Sara Ii — separator only
            i_v = i
            res.append("")
        elif ch in "\u0e32\u0e36\u0e37\u0e39\u0e45":
            # Sara Aa, Sara Ue, Sara Uee, Sara Uu, Lak Khang Yao — separator + encoded
            i_v = i
            res.append(ch.translate(_TRANS2))
        elif ch == "\u0e38":  # Sara U
            i_v = i
            if i == 0 or text[i - 1] not in "ตธ":
                res.append(ch.translate(_TRANS2))
            else:
                res.append("")
        elif ch in "\u0e2b\u0e2d":  # ห อ
            if i + 1 < n and text[i + 1] in "\u0e36\u0e37\u0e38\u0e39":
                res.append(ch.translate(_TRANS2))
        elif ch in "\u0e22\u0e23\u0e24\u0e26\u0e27":  # ย ร ฤ ฦ ว
            if i_v == i - 1 or (
                i + 1 < n and text[i + 1] in "\u0e36\u0e37\u0e38\u0e39"
            ):
                res.append(ch.translate(_TRANS2))
        else:
            res.append(ch.translate(_TRANS2))

    # Remove consecutive duplicates
    deduped: list[str] = [res[0]] if res else []
    for item in res[1:]:
        if item != deduped[-1]:
            deduped.append(item)

    return ("".join(deduped) + "0000")[:5]
