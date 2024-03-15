def test_dir_caching(spyn):
    code = """
import sys
print(sys.executable)
"""
    path = None
    for i in range(10):
        output = spyn.run_file(code=code, deps=["requests"])
        if i > 0:
            assert output == path
        else:
            path = output
