import google.protobuf.message
from google.protobuf.descriptor_pb2 import FileDescriptorSet
from google.protobuf.descriptor import FileDescriptor

from typing import Type, Set


def build_file_descriptor_set(
    message_class: Type[google.protobuf.message.Message],
) -> FileDescriptorSet:
    """
    Build a FileDescriptorSet representing the message class and its dependencies.
    """
    file_descriptor_set = FileDescriptorSet()
    seen_dependencies: Set[str] = set()

    def append_file_descriptor(file_descriptor: FileDescriptor) -> None:
        for dep in file_descriptor.dependencies:
            if dep.name not in seen_dependencies:
                seen_dependencies.add(dep.name)
                append_file_descriptor(dep)
        file_descriptor.CopyToProto(file_descriptor_set.file.add())

    append_file_descriptor(message_class.DESCRIPTOR.file)
    return file_descriptor_set
