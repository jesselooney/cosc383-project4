# Instructions

Uses poetry, install how you wish.

Once you have poetry, run `poetry install` to install dependencies. If you need to add a package, use `poetry add <PyPI package name>`. Run files with `poetry run python <file>` or by entering the virtual environment (which has its own copy of python with the correct dependencies) by running `poetry shell` and then calling `python3` as normal. I have included an autoformatter called Ruff in the dependencies, so run `poetry run ruff format` to format files before you commit.

Let's try to keep things in separate files (each of which is technically a Python "module") as much as possible, then we can import from these modules. You may want to look up how Python does modules/imports. Its mostly sensible but a little funky and I'm not sure I've ever fully felt like I understand, and it also can depend on directory structure.

Poetry docs:
https://python-poetry.org/docs/cli/#shell

