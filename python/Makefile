.PHONY: pipenv
pipenv:
	SETUPTOOLS_ENABLE_FEATURES="legacy-editable" pipenv install --dev --deploy

ifeq ($(shell uname),Darwin)
SED_ARGS = -i ''
else
SED_ARGS = -i
endif

.PHONY: generate-flatbuffer
generate-flatbuffer:
	find foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer ! -name '__init__.py' -type f -exec rm -f {} +
	pipenv run flatc \
		--python \
		-o foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer \
		../schemas/flatbuffer/*.fbs
	pipenv run flatc \
		-b \
		--schema \
		-o foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer \
		../schemas/flatbuffer/*.fbs
	rm foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer/foxglove/__init__.py
	mv foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer/foxglove/* foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer
	rmdir foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer/foxglove
	sed -E $(SED_ARGS) 's/from foxglove\./from \./g' foxglove-schemas-flatbuffer/foxglove_schemas_flatbuffer/*.py

.PHONY: generate-protobuf
generate-protobuf:
	rm -rf foxglove-schemas-protobuf/foxglove_schemas_protobuf/*_pb2*
	pipenv run protoc \
		--python_out=foxglove-schemas-protobuf/foxglove_schemas_protobuf \
		--mypy_out=foxglove-schemas-protobuf/foxglove_schemas_protobuf \
		--proto_path ../schemas/proto \
		../schemas/proto/foxglove/*.proto
	mv foxglove-schemas-protobuf/foxglove_schemas_protobuf/foxglove/* foxglove-schemas-protobuf/foxglove_schemas_protobuf
	rmdir foxglove-schemas-protobuf/foxglove_schemas_protobuf/foxglove
	sed -E $(SED_ARGS) 's/from foxglove import/from . import/g' foxglove-schemas-protobuf/foxglove_schemas_protobuf/*_pb2.py
	sed -E $(SED_ARGS) 's/import foxglove\.(.+)$$/from . import \1 as foxglove_\1/g' foxglove-schemas-protobuf/foxglove_schemas_protobuf/*_pb2.pyi
	sed -E $(SED_ARGS) 's/foxglove\./foxglove_/g' foxglove-schemas-protobuf/foxglove_schemas_protobuf/*_pb2.pyi

.PHONY: build
build: pipenv generate-flatbuffer generate-protobuf
	pipenv run python -m build foxglove-schemas-flatbuffer
	pipenv run python -m build foxglove-schemas-protobuf

.PHONY: test
test: pipenv generate-flatbuffer generate-protobuf
	pipenv run python -m pytest foxglove-schemas-flatbuffer
	pipenv run python -m pytest foxglove-schemas-protobuf

.PHONY: clean
clean:
	rm -rf dist
	find . -name "build" -type d -exec rm -rf {} +
	find . -name "dist" -type d -exec rm -rf {} +
	find . -name "*.egg-info" -type d -exec rm -rf {} +
	find . -name "*_pb2*" -delete
