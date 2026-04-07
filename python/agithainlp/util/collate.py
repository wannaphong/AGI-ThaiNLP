"""Thai collation (alphabetical sorting according to Thai dictionary order).

Reimplemented without using PyThaiNLP resources.
"""

from __future__ import annotations

import re
from collections.abc import Iterable

# Remove tonemarks for sort key
_RE_TONE: re.Pattern[str] = re.compile(r"[\u0e48\u0e49\u0e4a\u0e4b\u0e47\u0e4c]")
# Lead vowels (เ แ โ ใ ไ) come before the consonant visually but follow it phonetically
_RE_LEAD_VOWEL_CONSONANT: re.Pattern[str] = re.compile(r"([เ-ไ])([ก-ฮ])")


def _thkey(word: str) -> str:
    """Generate a sort key that follows Thai dictionary ordering.

    Tone marks are ignored (they don't affect ordering in Thai dictionaries).
    Lead vowels (เ แ โ ใ ไ) are swapped with the following consonant so
    that the consonant's position determines alphabetical order.
    """
    # Swap lead vowel + consonant → consonant + lead vowel
    key = _RE_LEAD_VOWEL_CONSONANT.sub(r"\2\1", word)
    # Strip tonemarks
    key = _RE_TONE.sub("", key)
    return key


def collate(data: Iterable[str], reverse: bool = False) -> list[str]:
    """Sort strings according to Thai dictionary order.

    Tonemarks and symbols are ignored during comparison.
    Lead vowels (เ แ โ ใ ไ) are treated as following their consonant.

    :param data: iterable of words to sort
    :type data: Iterable[str]
    :param reverse: if True, sort in descending order
    :type reverse: bool
    :return: sorted list of strings
    :rtype: list[str]

    :Example:

        >>> collate(['ไก่', 'เกิด', 'กาล', 'เป็ด', 'หมู', 'วัว', 'วันที่'])
        ['กาล', 'เกิด', 'ไก่', 'เป็ด', 'วันที่', 'วัว', 'หมู']
    """
    return sorted(data, key=_thkey, reverse=reverse)
