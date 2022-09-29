#!/usr/bin/env python3

import subprocess

query = subprocess.run(
    ['bazel', 'query', 'kind("proto_library", //...)'],
    capture_output=True,
    text=True,
    check=True)
proto_labels = frozenset(path for path in query.stdout.split())

for label in proto_labels:
    pkg, _, proto_rule = label.rpartition(':')
    assert proto_rule.endswith('_proto')
    proto_name = proto_rule[:-6]
    proto_cc_rule = proto_name + "_cc_proto"

    print(f"new cc_proto_library {proto_cc_rule} after {proto_rule}|{pkg}:__pkg__")
    print(f"add deps {pkg}:{proto_rule}|{pkg}:{proto_cc_rule}")
    print(f"set visibility //visibility:public|{pkg}:{proto_cc_rule}")
