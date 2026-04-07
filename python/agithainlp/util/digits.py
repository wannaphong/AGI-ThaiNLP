"""Thai digit and number utility functions.

Reimplemented without using PyThaiNLP resources.
"""

from __future__ import annotations

# Thai digit characters ๐-๙
_THAI_DIGITS: str = "๐๑๒๓๔๕๖๗๘๙"
_ARABIC_DIGITS: str = "0123456789"

_TH_TO_AR: dict[str, str] = dict(zip(_THAI_DIGITS, _ARABIC_DIGITS))
_AR_TO_TH: dict[str, str] = dict(zip(_ARABIC_DIGITS, _THAI_DIGITS))

# Thai words for digits and number components
_ONES: list[str] = [
    "ศูนย์", "หนึ่ง", "สอง", "สาม", "สี่",
    "ห้า", "หก", "เจ็ด", "แปด", "เก้า",
]
_TENS_SPECIAL: str = "ยี่"   # ยี่สิบ (twenty)
_TENS: str = "สิบ"
_HUNDRED: str = "ร้อย"
_THOUSAND: str = "พัน"
_TEN_THOUSAND: str = "หมื่น"
_HUNDRED_THOUSAND: str = "แสน"
_MILLION: str = "ล้าน"
_ET: str = "เอ็ด"   # final unit position "one"

# Reverse mapping for Thai words → numbers
_WORD_TO_DIGIT: dict[str, int] = {
    "ศูนย์": 0,
    "หนึ่ง": 1,
    "สอง": 2,
    "สาม": 3,
    "สี่": 4,
    "ห้า": 5,
    "หก": 6,
    "เจ็ด": 7,
    "แปด": 8,
    "เก้า": 9,
    "เอ็ด": 1,
    "ยี่": 2,
}
_WORD_TO_MULTIPLIER: dict[str, int] = {
    "สิบ": 10,
    "ร้อย": 100,
    "พัน": 1_000,
    "หมื่น": 10_000,
    "แสน": 100_000,
    "ล้าน": 1_000_000,
}


def thai_digit_to_arabic_digit(text: str) -> str:
    """Convert Thai digit characters to Arabic digit characters.

    :param str text: text containing Thai digits
    :return: text with Thai digits replaced by Arabic digits
    :rtype: str

    :Example:

        >>> thai_digit_to_arabic_digit("๑๒๓")
        '123'
    """
    return "".join(_TH_TO_AR.get(ch, ch) for ch in text)


def arabic_digit_to_thai_digit(text: str) -> str:
    """Convert Arabic digit characters to Thai digit characters.

    :param str text: text containing Arabic digits
    :return: text with Arabic digits replaced by Thai digits
    :rtype: str

    :Example:

        >>> arabic_digit_to_thai_digit("123")
        '๑๒๓'
    """
    return "".join(_AR_TO_TH.get(ch, ch) for ch in text)


def num_to_thaiword(number: int) -> str:
    """Convert an integer to its Thai word representation.

    Supports non-negative integers up to 999,999,999.

    :param int number: non-negative integer
    :return: Thai word representation
    :rtype: str

    :Example:

        >>> num_to_thaiword(0)
        'ศูนย์'
        >>> num_to_thaiword(21)
        'ยี่สิบเอ็ด'
        >>> num_to_thaiword(1000000)
        'หนึ่งล้าน'
    """
    if not isinstance(number, int) or number < 0:
        raise ValueError("number must be a non-negative integer")

    if number == 0:
        return _ONES[0]

    def _chunk(n: int) -> str:
        """Convert a number < 1,000,000 to Thai words."""
        if n == 0:
            return ""
        parts: list[str] = []
        h = n // 100_000
        n %= 100_000
        tm = n // 10_000
        n %= 10_000
        th = n // 1_000
        n %= 1_000
        hu = n // 100
        n %= 100
        te = n // 10
        on = n % 10

        if h:
            parts.append(_ONES[h] + _HUNDRED_THOUSAND)
        if tm:
            parts.append(_ONES[tm] + _TEN_THOUSAND)
        if th:
            parts.append(_ONES[th] + _THOUSAND)
        if hu:
            parts.append(_ONES[hu] + _HUNDRED)
        if te:
            if te == 2:
                parts.append(_TENS_SPECIAL + _TENS)
            elif te == 1:
                parts.append(_TENS)
            else:
                parts.append(_ONES[te] + _TENS)
        if on:
            # "เอ็ด" is used instead of "หนึ่ง" when in final units position
            # with a tens prefix
            if on == 1 and te:
                parts.append(_ET)
            else:
                parts.append(_ONES[on])
        return "".join(parts)

    millions = number // 1_000_000
    remainder = number % 1_000_000

    result = ""
    if millions:
        result += _chunk(millions) + _MILLION
    if remainder:
        result += _chunk(remainder)
    return result


def thaiword_to_num(text: str) -> int | None:
    """Convert a Thai number word to an integer.

    Handles basic Thai number words (ศูนย์ through ล้าน).

    :param str text: Thai number word
    :return: integer value, or None if conversion failed
    :rtype: int | None

    :Example:

        >>> thaiword_to_num("ยี่สิบเอ็ด")
        21
        >>> thaiword_to_num("หนึ่งร้อยห้าสิบ")
        150
    """
    if not text or not isinstance(text, str):
        return None

    # Tokenize text into a list of (value, is_multiplier) pairs
    all_tokens: list[tuple[str, int, bool]] = []
    for word, val in _WORD_TO_DIGIT.items():
        all_tokens.append((word, val, False))
    for word, val in _WORD_TO_MULTIPLIER.items():
        all_tokens.append((word, val, True))

    # Greedy tokenize from left to right
    tokens: list[tuple[int, bool]] = []
    pos = 0
    n = len(text)
    while pos < n:
        matched = False
        for word, val, is_mult in sorted(
            all_tokens, key=lambda t: len(t[0]), reverse=True
        ):
            if text[pos:].startswith(word):
                tokens.append((val, is_mult))
                pos += len(word)
                matched = True
                break
        if not matched:
            return None

    # Parse the token list hierarchically.
    # Strategy: walk left-to-right maintaining a running value for the
    # current "group" (before ล้าน) and a grand total.
    # When a multiplier is seen, it applies only to the digit(s) directly
    # before it at the same level.
    #
    # Example: [1, x100, 5, x10]
    #   1 * 100 = 100   (group resets to 100)
    #   5 * 10  =  50   (add 50 → total becomes 150)
    total = 0
    group = 0   # accumulated value within current ล้าน group
    pending = 0  # value waiting for a multiplier

    for val, is_mult in tokens:
        if not is_mult:
            # Digit: add to pending
            pending += val
        else:
            if val == 1_000_000:
                # ล้าน resets the group
                group_val = group + pending if (group or pending) else 1
                total += group_val * 1_000_000
                group = 0
                pending = 0
            else:
                # Regular multiplier (10, 100, 1000, 10000, 100000)
                # Applies only to pending (the digit(s) just seen)
                factor = pending if pending else 1
                group += factor * val
                pending = 0

    return total + group + pending
