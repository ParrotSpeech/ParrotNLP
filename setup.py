from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="parrotnlp",
    rust_extensions=[
        RustExtension(
            "parrotnlp._parrotnlp",
            binding=Binding.PyO3,
            # Between our use of editable installs and wanting to use Rust for performance sensitive
            # code, it makes sense to just always use --release
            debug=False,
            features=["python"],
        )
    ],
    package_data={"parrotnlp": ["py.typed"]},
    packages=["parrotnlp"],
    zip_safe=False,
)
