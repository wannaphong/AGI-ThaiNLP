"""Tokenization sub-package for AGI-ThaiNLP."""

from agithainlp.tokenize.tcc import tcc_tokenize, syllable_tokenize
from agithainlp.tokenize.sent import sent_tokenize

__all__ = ["tcc_tokenize", "syllable_tokenize", "sent_tokenize"]
