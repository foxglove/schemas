# Foxglove Python SDK

## Unstable

This package is currently under active development and not recommended for production use. See [foxglove-websocket](https://github.com/foxglove/ws-protocol/tree/main/python) for the currently recommended approach.

## Development

### Installation

We use [Poetry](https://python-poetry.org/) to manage dependencies.

Install Poetry:

```sh
brew install pipx
pipx ensurepath
pipx install poetry
```

Install dependencies

```sh
poetry install
```

### Developing

To make use of installed dependencies, prefix python commands with `poetry run`. For more details, refer to the [Poetry docs](https://python-poetry.org/docs/basic-usage/).

After making changes to rust code, rebuild with:

```sh
poetry run maturin develop
```

To check types, run:

```sh
poetry run mypy .
```

Format code:

```sh
poetry run black .
```

PEP8 check:

```sh
poetry run flake8 .
```

### Examples

```sh
poetry run python python/examples/add.py
poetry run python python/examples/server.py
```
