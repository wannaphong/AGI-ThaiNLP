"""Thai sentence tokenizer.

Rule-based sentence segmentation for Thai text.
Sentences are split on whitespace sequences and sentence-ending markers
such as Thai punctuation characters, newlines, and common patterns.
"""

from __future__ import annotations

import re

# Split after ASCII sentence-ending punctuation (. ! ?) followed by whitespace
# and then a capital letter or Thai character (start of new sentence).
# Also split on newlines and Thai sentence-end marks.
_SENT_END: re.Pattern[str] = re.compile(
    r"(?<=[.!?])\s+(?=[A-ZÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖØÙÚÛÜÝÞŸ\u0e00-\u0e7f])"
    r"|(?<=[๚๛ฯ])\s*"   # Thai sentence-end marks
    r"|[\r\n]+"           # newlines
)


def sent_tokenize(text: str) -> list[str]:
    """Tokenize Thai (and mixed) text into sentences.

    This rule-based tokenizer splits on:
    * newlines
    * ASCII sentence-ending punctuation (. ! ?) followed by whitespace and
      a new sentence (uppercase letter or Thai character)
    * Thai sentence-ending markers (๚ ๛ ฯ)

    :param str text: text to split into sentences
    :return: list of sentence strings
    :rtype: list[str]

    :Example:

        >>> sent_tokenize("วันนี้ฉันไปตลาด วันพรุ่งนี้ฉันจะไปโรงเรียน")
        ['วันนี้ฉันไปตลาด วันพรุ่งนี้ฉันจะไปโรงเรียน']
        >>> sent_tokenize("I love cats. Do you?\\nYes I do.")
        ['I love cats.', 'Do you?', 'Yes I do.']
    """
    if not text or not isinstance(text, str):
        return []

    parts = [s.strip() for s in _SENT_END.split(text)]
    return [p for p in parts if p]
