import pytest
import tempfile
import pathlib
import subprocess


class Spyn:
    def __init__(self, binary_path):
        self.binary = binary_path

    def run_file(self, *, path=None, contents=None, deps=()):
        cmd = f"{self.binary} "
        assert (path is not None) ^ (contents is not None)
        if path is not None:
            cmd += path
        elif contents is not None:
            with open(tempfile.mktemp(), "w", encoding="utf-8") as f:
                f.write(contents)
            cmd += f.name
        for dep in deps:
            cmd += f" -d {dep}"
        return subprocess.check_output(cmd, shell=True, cwd="/tmp", encoding="utf-8")


PROJ_ROOT = pathlib.Path(__file__).parent.parent.absolute()


@pytest.fixture
def spyn():
    subprocess.check_call("cargo build", shell=True, cwd=PROJ_ROOT)
    return Spyn(PROJ_ROOT / "target/debug/spyn")
