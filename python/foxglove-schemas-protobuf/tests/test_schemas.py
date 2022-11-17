def test_schemas():
    from foxglove_schemas_protobuf.RawImage_pb2 import RawImage

    img = RawImage()
    img.encoding = "png"
