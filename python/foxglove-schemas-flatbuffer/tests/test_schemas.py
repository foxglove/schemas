def test_schemas():
    import flatbuffers
    import foxglove_schemas_flatbuffer.RawImage as RawImage

    builder = flatbuffers.Builder(1024)
    png = builder.CreateString("png")
    RawImage.Start(builder)
    RawImage.AddEncoding(builder, png)
    img = RawImage.End(builder)
    builder.Finish(img)
    buf = builder.Output()
    assert buf is not None
    assert len(buf) > 0


def test_get_schema():
    from foxglove_schemas_flatbuffer import get_schema

    schema = get_schema("RawImage")
    assert schema is not None
    assert len(schema) > 0
