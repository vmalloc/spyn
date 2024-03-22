import pytest


@pytest.mark.parametrize("marker", ["spyn", "fades"])
def test_run_marker(spyn, marker):
    assert (
        spyn.run_file(
            code=f"""
import requests   # {marker}          
print('success')
"""
        )
        == "success\n"
    )


def test_run_dep(spyn):
    assert (
        spyn.run_file(
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
        spyn.run_file(
            code="""
import requests
print('success')
""",
            args=["-r", f.name],
        )
        == "success\n"
    )
