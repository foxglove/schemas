import importlib.resources as resources


def get_schema(name: str) -> bytes:
    return resources.read_binary(__package__, name + ".bfbs")
