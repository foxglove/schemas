#!/bin/bash

# MacOS:
# brew install protobuf@3
# export PATH="/opt/homebrew/opt/protobuf@3/bin:$PATH"
# export LDFLAGS="-L/opt/homebrew/opt/protobuf@3/lib"
# export CPPFLAGS="-I/opt/homebrew/opt/protobuf@3/include"

# All Platforms:
# go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
# go install github.com/goreleaser/goreleaser@latest
# export PATH="$PATH:$(go env GOPATH)/bin"

PROJDIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")") # Get the project directory

cd "$PROJDIR" || exit 1

protoc --go_out=./ --proto_path=schemas/proto schemas/proto/**/*.proto --descriptor_set_out=/dev/null