import importlib.resources as pkg_resources


def get_schema(name: str) -> bytes:
    return pkg_resources.read_binary(__package__, name + ".bfbs")
