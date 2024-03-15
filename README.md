# What is Spyn?

Spyn is a tool for creating on-demand, ad-hoc virtual environments for running Python applications.

Spyn was heavily inspired by [fades](https://fades.readthedocs.io/en/latest/index.html), but tries to improve upon the same concept:
1. Spyn is written in Rust and comes as a binary executable, meaning you don't need to install it through script wrappers or taint your local Python installation. It will also not break when system Python versions get upgraded or reinstalled. Spyn also runs much quicker thanks to that fact
2. Spyn uses `uv` for creating and managing virtual environments. This allows it to create virtuelenvs in fractions of seconds and be less prone to `pip` breakages and hiccups.

# Installation
You can use `cargo install` to install `spyn`:

```
$ cargo install spyn
```

# Usage

You can use `spyn` to run a Python script, specifying the dependencies it needs:

```
$ spyn -d requests ./my_script.py
```

You can also mark dependencies in the script itself which are to be installed by spyn:

```
$ cat my_script.py
import os

import requests # spyn

print("Hello, World!")
$ spyn ./my_script.py
```
`spyn` even accepts lines marked with `fades` to provide backwards compatibility with [fades](https://fades.readthedocs.io/en/latest/index.html)!

If you want an interactive session with custom dependencies, `spyn` supports the `--ipython` flag, dropping you into an IPython shell with the dependencies you specify:

```
$ spyn --ipython -d requests
Using Python 3.11.2 interpreter at: /Users/rotemy/src/oss/spyn/.venv/bin/python3
Creating virtualenv at: /var/folders/bs/wqjbrbn948j78gb4303rjdcc0000gn/T/spyn.FDlEhIQvFyXR
Activate with: source /var/folders/bs/wqjbrbn948j78gb4303rjdcc0000gn/T/spyn.FDlEhIQvFyXR/bin/activate
Resolved 21 packages in 451ms
Installed 21 packages in 82ms
...

In [1]: import requests
In [2]: 
```