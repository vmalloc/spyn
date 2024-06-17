import pytest


@pytest.mark.parametrize("marker", ["spyn", "fades"])
def test_run_marker(spyn, marker):
    assert (
        spyn.run(
            code=f"""
import requests   # {marker}          
print('success')
"""
        )
        == "success\n"
    )


def test_run_dep(spyn):
    assert (
        spyn.run(
            code="""
import requests
print('success')
""",
            deps=["requests"],
        )
        == "success\n"
    )


def test_req_file(spyn, tmpdir):
    with (tmpdir / "reqfile.txt").open("w") as f:
        print("requests", file=f)

    assert (
        spyn.run(
            code="""
import requests
print('success')
""",
            args=["-r", f.name],
        )
        == "success\n"
    )


def test_run_x_cmd(spyn, tmpdir):
    dirname = tmpdir / "tests"
    dirname.mkdir()
    with (dirname / "test_something.py").open("w") as f:
        f.write("def test_something(): pass")
    assert "1 passed" in spyn.run(
        x_cmd="pytest",
        args=[dirname],
        deps=["pytest"],
    )
