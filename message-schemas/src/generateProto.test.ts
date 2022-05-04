import protobufjs from "protobufjs";

import { DURATION_PROTO, generateProto, TIME_PROTO } from "./generateProto";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "./schemas";
import { exampleEnum, exampleMessage } from "./testFixtures";

describe("generateProtoFiles", () => {
  it("generates .proto files", () => {
    expect(generateProto(exampleEnum)).toMatchInlineSnapshot(`
      "// Generated from ExampleEnum by @foxglove/message-schemas

      syntax = \\"proto3\\";

      package foxglove;

      // An example enum
      enum ExampleEnum {
        // Value A
        A = 1;

        // Value B
        B = 2;
      }
      "
    `);
    expect(generateProto(exampleMessage)).toMatchInlineSnapshot(`
      "// Generated from ExampleMessage by @foxglove/message-schemas

      syntax = \\"proto3\\";

      import \\"foxglove/Duration.proto\\";
      import \\"foxglove/ExampleEnum.proto\\";
      import \\"foxglove/NestedMessage.proto\\";
      import \\"foxglove/Time.proto\\";

      package foxglove;

      // An example type
      message ExampleMessage {
        // Duration field
        foxglove.Duration field_duration = 1;

        // Time field
        foxglove.Time field_time = 2;

        // boolean field
        bool field_boolean = 3;

        // bytes field
        bytes field_bytes = 4;

        // float field
        double field_float = 5;

        // integer field
        int32 field_integer = 6;

        // string field
        string field_string = 7;

        // Duration array field
        repeated foxglove.Duration field_duration_array = 8;

        // Time array field
        repeated foxglove.Time field_time_array = 9;

        // boolean array field
        repeated bool field_boolean_array = 10;

        // bytes array field
        repeated bytes field_bytes_array = 11;

        // float array field
        repeated double field_float_array = 12;

        // integer array field
        repeated int32 field_integer_array = 13;

        // string array field
        repeated string field_string_array = 14;

        // An enum field
        foxglove.ExampleEnum field_enum = 15;

        // An enum array field
        repeated foxglove.ExampleEnum field_enum_array = 16;

        // A nested field
        foxglove.NestedMessage field_nested = 17;

        // A nested array field
        repeated foxglove.NestedMessage field_nested_array = 18;
      }
      "
    `);
  });

  it("generates parseable .proto files", () => {
    const root = new protobufjs.Root();
    root.add(protobufjs.parse(TIME_PROTO).root);
    root.add(protobufjs.parse(DURATION_PROTO).root);
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      root.add(protobufjs.parse(generateProto(schema)).root);
    }
    for (const schema of Object.values(foxgloveEnumSchemas)) {
      root.add(protobufjs.parse(generateProto(schema)).root);
    }
    expect(() => root.resolveAll()).not.toThrow();
  });
});
