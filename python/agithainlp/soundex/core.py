"""Core soundex dispatcher for AGI-ThaiNLP."""

from __future__ import annotations

from agithainlp.soundex.udom83 import udom83
from agithainlp.soundex.lk82 import lk82

_DEFAULT_ENGINE: str = "udom83"


def soundex(text: str, engine: str = _DEFAULT_ENGINE) -> str:
    """Convert Thai text to phonetic code.

    :param str text: Thai word
    :param str engine: soundex engine to use. Options:

        * ``"udom83"`` (default) — Udom83 system, returns 7-character code
        * ``"lk82"`` — LK82 system, returns 5-character code

    :return: soundex code
    :rtype: str

    :Example:

        >>> soundex("รัก")
        'ร100000'
        >>> soundex("รัก", engine="lk82")
        'ร1000'
    """
    if engine == "lk82":
        return lk82(text)
    return udom83(text)
