import sys
import pytest
import subprocess

_GET_INTERPRETER = """
import sys
print(sys.executable)
"""


def test_dir_caching(spyn):
    path = None
    for i in range(10):
        output = spyn.run(code=_GET_INTERPRETER, deps=["requests"])
        if i > 0:
            assert output == path
        else:
            path = output


def test_dir_caching_different_python_versions(spyn):

    path = None
    deps = ["requests"]
    for i in range(10):
        output = spyn.run(code=_GET_INTERPRETER, deps=deps)
        if i > 0:
            assert output == path
        else:
            path = output

    # even though it's the same python executable, it should be cached separately
    new_path = spyn.run(code=_GET_INTERPRETER, deps=deps, python=sys.version.split()[0])
    assert new_path != path
