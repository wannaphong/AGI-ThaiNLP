"""Thai text normalization utilities.

Reimplemented without using PyThaiNLP resources.
"""

from __future__ import annotations

import re

# Tonemark characters: ่ ้ ๊ ๋
_RE_TONEMARK: re.Pattern[str] = re.compile(r"[\u0e48\u0e49\u0e4a\u0e4b]")


def remove_tonemark(text: str) -> str:
    """Remove Thai tonemarks from text.

    Removes the four Thai tonemarks: ่ (mai ek), ้ (mai tho),
    ๊ (mai tri), ๋ (mai jattawa).

    :param str text: Thai text
    :return: text with tonemarks removed
    :rtype: str

    :Example:

        >>> remove_tonemark("น้ำ")
        'นำ'
        >>> remove_tonemark("ข้าว")
        'ขาว'
    """
    if not text or not isinstance(text, str):
        return text
    return _RE_TONEMARK.sub("", text)
