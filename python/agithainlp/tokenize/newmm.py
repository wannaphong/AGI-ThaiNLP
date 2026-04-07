"""NewMM (New Maximum Matching) Thai word tokenizer.

Implements the Maximum Matching algorithm with a Viterbi (DP) path-finding
step.  For each input position the algorithm considers all dictionary words
that start there and, as a fallback, the smallest Thai Character Cluster
(TCC) unit.  Dynamic programming is used to select the segmentation that
minimises the total number of tokens (equivalent to maximum matching).

This is a reimplementation without using PyThaiNLP resources.  A compact
built-in word list is supplied in :mod:`agithainlp.tokenize._words`; callers
can override it via the ``custom_dict`` parameter.

References:
    * Haruechaiyasak, C. et al. (2009). A Comparative Study on Thai Word
      Segmentation Approaches.
    * Original PyThaiNLP implementation:
      https://github.com/PyThaiNLP/pythainlp/blob/dev/pythainlp/tokenize/newmm.py
"""

from __future__ import annotations

from agithainlp.tokenize._words import THAI_WORDS
from agithainlp.tokenize.tcc import tcc_tokenize

# ---------------------------------------------------------------------------
# Trie
# ---------------------------------------------------------------------------

class _TrieNode:
    """Single node of a prefix trie."""

    __slots__ = ("children", "is_end")

    def __init__(self) -> None:
        self.children: dict[str, _TrieNode] = {}
        self.is_end: bool = False


class _Trie:
    """Prefix trie for O(k) word-prefix lookup (k = word length)."""

    def __init__(self, words: frozenset[str] | set[str] | list[str]) -> None:
        self._root = _TrieNode()
        for w in words:
            self._add(w)

    def _add(self, word: str) -> None:
        node = self._root
        for ch in word:
            if ch not in node.children:
                node.children[ch] = _TrieNode()
            node = node.children[ch]
        node.is_end = True

    def prefixes(self, text: str) -> list[str]:
        """Return all words in the trie that are prefixes of *text*.

        Returned in increasing length order.
        """
        node = self._root
        results: list[str] = []
        for i, ch in enumerate(text):
            if ch not in node.children:
                break
            node = node.children[ch]
            if node.is_end:
                results.append(text[: i + 1])
        return results


# Cache one trie per frozenset so repeated calls with the same dict are cheap.
_TRIE_CACHE: dict[int, _Trie] = {}
_DEFAULT_TRIE: _Trie | None = None


def _get_trie(custom_dict: frozenset[str] | None) -> _Trie:
    global _DEFAULT_TRIE
    if custom_dict is None:
        if _DEFAULT_TRIE is None:
            _DEFAULT_TRIE = _Trie(THAI_WORDS)
        return _DEFAULT_TRIE
    key = id(custom_dict)
    if key not in _TRIE_CACHE:
        _TRIE_CACHE[key] = _Trie(custom_dict)
    return _TRIE_CACHE[key]


# ---------------------------------------------------------------------------
# NewMM segmentation
# ---------------------------------------------------------------------------

def _newmm_segment(text: str, trie: _Trie) -> list[str]:
    """Core NewMM DP segmentation on a single (Thai) text chunk.

    Uses a minimum-cut DP: ``dp[j]`` holds the fewest tokens needed to cover
    ``text[:j]``.  Both dictionary words and TCC fallback units are tried at
    every position; the DP naturally selects the longest matching word
    (maximum matching) because longer words require fewer total cuts.
    """
    n = len(text)
    INF = n + 1

    # dp[i] = minimum number of tokens to cover text[0:i]
    dp: list[int] = [INF] * (n + 1)
    dp[0] = 0
    # prev[i] = start position of the last token that ends at i
    prev: list[int] = [-1] * (n + 1)

    for i in range(n):
        if dp[i] == INF:
            continue

        cost = dp[i] + 1

        # 1. Try all dictionary words starting at position i
        for word in trie.prefixes(text[i:]):
            j = i + len(word)
            if cost < dp[j]:
                dp[j] = cost
                prev[j] = i

        # 2. TCC fallback — always offer one TCC unit so the DP never gets stuck
        tcc_units = tcc_tokenize(text[i:])
        unit = tcc_units[0] if tcc_units else text[i]
        j = i + len(unit)
        if cost < dp[j]:
            dp[j] = cost
            prev[j] = i

    # Reconstruct path
    if dp[n] == INF:
        return [text]

    tokens: list[str] = []
    pos = n
    while pos > 0:
        start = prev[pos]
        tokens.append(text[start:pos])
        pos = start
    tokens.reverse()
    return tokens


def word_tokenize(
    text: str,
    custom_dict: frozenset[str] | None = None,
    keep_whitespace: bool = True,
) -> list[str]:
    """Tokenize Thai (and mixed) text into words using the NewMM algorithm.

    NewMM is a Maximum Matching tokenizer augmented with TCC-based fallback
    for unknown words.  It uses dynamic programming to find the segmentation
    with the fewest tokens (equivalent to maximum-length matching).

    :param str text: text to tokenize
    :param custom_dict: optional set of words to use instead of the
        built-in word list.  Pass a :class:`frozenset` for best performance.
    :type custom_dict: frozenset[str] | None
    :param bool keep_whitespace: if ``True`` (default), whitespace tokens
        are preserved in the output list
    :return: list of word tokens
    :rtype: list[str]

    :Example:

        >>> word_tokenize("ผมชอบกินข้าวผัดกุ้ง")
        ['ผม', 'ชอบ', 'กิน', 'ข้าว', 'ผัด', 'กุ้ง']
        >>> word_tokenize("วันนี้อากาศดีมาก")
        ['วันนี้', 'อากาศ', 'ดี', 'มาก']
        >>> word_tokenize("I love ข้าวผัด")
        ['I', ' ', 'love', ' ', 'ข้าว', 'ผัด']
    """
    if not text or not isinstance(text, str):
        return []

    trie = _get_trie(custom_dict)
    result: list[str] = []
    i = 0
    n = len(text)

    while i < n:
        ch = text[i]
        if ch == " " or ch == "\t":
            # Whitespace run
            j = i
            while j < n and text[j] in (" ", "\t"):
                j += 1
            ws = text[i:j]
            if keep_whitespace:
                result.append(ws)
            i = j
        elif ch == "\n" or ch == "\r":
            if keep_whitespace:
                result.append(ch)
            i += 1
        elif "\u0e00" <= ch <= "\u0e7f":
            # Thai run: collect consecutive Thai characters and segment
            j = i
            while j < n and "\u0e00" <= text[j] <= "\u0e7f":
                j += 1
            thai_chunk = text[i:j]
            result.extend(_newmm_segment(thai_chunk, trie))
            i = j
        else:
            # Non-Thai, non-whitespace run (ASCII words, numbers, punctuation)
            j = i
            while (
                j < n
                and text[j] not in (" ", "\t", "\n", "\r")
                and not ("\u0e00" <= text[j] <= "\u0e7f")
            ):
                j += 1
            result.append(text[i:j])
            i = j

    return result
