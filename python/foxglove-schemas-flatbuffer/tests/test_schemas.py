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
