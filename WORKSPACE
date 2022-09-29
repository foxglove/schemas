workspace(name = "com_github_foxglove_schemas")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "com_google_protobuf",
    sha256 = "dbb16fdbca8f277c9a194d9a837395cde408ca136738d94743130dd0de015efd",
    strip_prefix = "protobuf-21.6",
    urls = [
        "https://mirror.bazel.build/github.com/protocolbuffers/protobuf/archive/refs/tags/v21.6.tar.gz",
        "https://github.com/protocolbuffers/protobuf/archive/refs/tags/v21.6.tar.gz",
    ],
)

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

protobuf_deps()
