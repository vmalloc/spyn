import os
import pathlib
import subprocess
import tempfile

import pytest


class SpynError(Exception):
    pass


class Spyn:
    def __init__(self, binary_path):
        self.binary = binary_path

    def run_file(self, *, path=None, code=None, deps=()):
        cmd = f"{self.binary} "
        assert (path is not None) ^ (code is not None)
        if path is not None:
            cmd += path
        elif code is not None:
            with open(tempfile.mktemp(), "w", encoding="utf-8") as f:
                f.write(code)
            cmd += f.name
        for dep in deps:
            cmd += f" -d {dep}"
        try:
            return subprocess.check_output(
                cmd,
                shell=True,
                cwd="/tmp",
                encoding="utf-8",
                env={"RUST_LOG": "DEBUG", **os.environ},
            )
        except subprocess.CalledProcessError as e:
            raise SpynError(f"command {cmd} failed: {e.output}")


PROJ_ROOT = pathlib.Path(__file__).parent.parent.absolute()


@pytest.fixture
def spyn():
    subprocess.check_call("cargo build", shell=True, cwd=PROJ_ROOT)
    return Spyn(PROJ_ROOT / "target/debug/spyn")
