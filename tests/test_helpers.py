import bisect
import functools
import os

import pytest

import parrotnlp

MAX_EXAMPLES: int = int(os.environ.get("parrotnlp_MAX_EXAMPLES", "100"))

ENCODINGS = ["r50k_base", "cl100k_base"]
SOME_ENCODINGS = ["cl100k_base"]


ENCODING_FACTORIES = [
    pytest.param(functools.partial(parrotnlp.get_encoding, name), id=name) for name in ENCODINGS
]
SOME_ENCODING_FACTORIES = [
    pytest.param(functools.partial(parrotnlp.get_encoding, name), id=name) for name in SOME_ENCODINGS
]


