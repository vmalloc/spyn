import pytest


@pytest.mark.parametrize("marker", ["spyn", "fades"])
def test_run_marker(spyn, marker):
    assert (
        spyn.run_file(
            contents=f"""
import requests   # {marker}          
print('success')
"""
        )
        == "success\n"
    )


def test_run_dep(spyn):
    assert (
        spyn.run_file(
            contents="""
import requests
print('success')
""",
            deps=["requests"],
        )
        == "success\n"
    )
