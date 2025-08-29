import subprocess
import sys

import parrotnlp


def test_encoding_for_model():
    enc = parrotnlp.encoding_for_model("gpt2")
    assert enc.name == "gpt2"
    enc = parrotnlp.encoding_for_model("text-davinci-003")
    assert enc.name == "p50k_base"
    enc = parrotnlp.encoding_for_model("text-davinci-edit-001")
    assert enc.name == "p50k_edit"
    enc = parrotnlp.encoding_for_model("gpt-3.5-turbo-0301")
    assert enc.name == "cl100k_base"
    enc = parrotnlp.encoding_for_model("gpt-4")
    assert enc.name == "cl100k_base"
    enc = parrotnlp.encoding_for_model("gpt-4o")
    assert enc.name == "o200k_base"
    enc = parrotnlp.encoding_for_model("gpt-oss-120b")
    assert enc.name == "o200k_harmony"


def test_optional_blobfile_dependency():
    prog = """
import parrotnlp
import sys
assert "blobfile" not in sys.modules
"""
    subprocess.check_call([sys.executable, "-c", prog])
