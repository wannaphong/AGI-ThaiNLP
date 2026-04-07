"""Thai Character Cluster (TCC) tokenizer.

Implements the TCC rules proposed by Theeramunkong et al. 2000.
This is a reimplementation without using PyThaiNLP resources.

References:
    * Theeramunkong, T. et al. (2000). Character cluster based Thai
      information retrieval. IRAL 2000.
    * Grammar by Wittawat Jitkrittum
      https://github.com/wittawatj/jtcc/blob/master/TCC.g
"""

from __future__ import annotations

import re
from collections.abc import Iterator


# Thai consonant class [ก-ฮ]
_C = r"[ก-ฮ]"
# Optional tone mark class [่-๋]
_T = r"[่-๋]?"
# Killing character / Thanthakhat
_K = r"(cc?[ิุู]?[์])?"

# Build TCC regex patterns (derived from Theeramunkong et al. 2000 rules)
# The patterns below encode the well-formed Thai Character Clusters.
_RE_TCC_PATTERNS: list[str] = (
    r"""c[ั]([่-๋]c)?
c[ั]([่-๋]c)?k
เc็ck
เcctาะk
เccีtยะk
เccีtย(?=[เ-ไก-ฮ]|$)k
เc[ิีุู]tย(?=[เ-ไก-ฮ]|$)k
เcc็ck
เcิc์ck
เcิtck
เcีtยะ?k
เcืtอะk
เcื
เctา?ะ?k
c[ึื]tck
c[ะ-ู]tk
c[ิุู]์
cรรc์
c็
ct[ะาำ]?k
แc็ck
แcc์k
แctะk
แcc็ck
แccc์k
โctะk
[เ-ไ]ctk
ก็
อึ
หึ
"""
    .replace("k", r"(cc?[ิุู]?[์])?")
    .replace("c", r"[ก-ฮ]")
    .replace("t", r"[่-๋]?")
    .split()
)

_PAT_TCC: re.Pattern[str] = re.compile("|".join(_RE_TCC_PATTERNS))


def tcc_tokenize(text: str) -> list[str]:
    """Tokenize Thai text into Thai Character Clusters (TCC).

    A Thai Character Cluster (TCC) is a group of characters that
    cannot be separated. This is the smallest meaningful unit for
    Thai text segmentation.

    :param str text: text to tokenize
    :return: list of Thai Character Clusters
    :rtype: list[str]

    :Example:

        >>> tcc_tokenize("ประเทศไทย")
        ['ป', 'ระ', 'เท', 'ศ', 'ไท', 'ย']
        >>> tcc_tokenize("กาแฟ")
        ['กา', 'แฟ']
    """
    return list(_tcc_gen(text))


def syllable_tokenize(text: str) -> list[str]:
    """Tokenize Thai text into syllables (TCC-based).

    This is an alias for :func:`tcc_tokenize` since TCC units
    approximate syllable-level segmentation.

    :param str text: text to tokenize
    :return: list of syllables
    :rtype: list[str]

    :Example:

        >>> syllable_tokenize("ประเทศไทย")
        ['ป', 'ระ', 'เท', 'ศ', 'ไท', 'ย']
    """
    return tcc_tokenize(text)


def _tcc_gen(text: str) -> Iterator[str]:
    """Generator that yields Thai Character Cluster tokens.

    :param str text: text to tokenize
    :return: iterator of TCC tokens
    :rtype: Iterator[str]
    """
    if not text or not isinstance(text, str):
        return

    n = len(text)
    pos = 0
    while pos < n:
        match = _PAT_TCC.match(text[pos:])
        if match:
            span = match.span()[1]
        else:
            span = 1
        yield text[pos : pos + span]
        pos += span
