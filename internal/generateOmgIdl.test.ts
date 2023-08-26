import { parseIdlToMessageDefinition } from "@foxglove/omgidl-parser";

import { DURATION_IDL, TIME_IDL, generateOmgIdl } from "./generateOmgIdl";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "./schemas";
import { exampleEnum, exampleMessage } from "./testFixtures";

describe("generateOmgIdl", () => {
  it("generates .idl files", () => {
    expect(generateOmgIdl(exampleEnum)).toMatchInlineSnapshot(`
      "// Generated by https://github.com/foxglove/schemas

      module foxglove {

      // An example enum
      enum ExampleEnum {
        // Value A
        // Value: 0
        A,

        // Value B
        // Value: 1
        B
      };

      };
      "
    `);
    expect(generateOmgIdl(exampleMessage)).toMatchInlineSnapshot(`
      "// Generated by https://github.com/foxglove/schemas

      #include "foxglove/Duration.idl"
      #include "foxglove/ExampleEnum.idl"
      #include "foxglove/NestedMessage.idl"
      #include "foxglove/Time.idl"

      module foxglove {

      // An example type
      struct ExampleMessage {
        // duration field
        Duration field_duration;

        // time field
        Time field_time;

        // boolean field
        boolean field_boolean;

        // bytes field
        sequence<uint8> field_bytes;

        // float64 field
        double field_float64;

        // uint32 field
        uint32 field_uint32;

        // string field
        string field_string;

        // duration array field
        sequence<Duration> field_duration_array;

        // time array field
        sequence<Time> field_time_array;

        // boolean array field
        sequence<boolean> field_boolean_array;

        // bytes array field
        sequence<sequence<uint8>> field_bytes_array;

        // float64 array field
        sequence<double> field_float64_array;

        // uint32 array field
        sequence<uint32> field_uint32_array;

        // string array field
        sequence<string> field_string_array;

        // duration fixed-length array field
        Duration field_duration_fixed_array[3];

        // time fixed-length array field
        Time field_time_fixed_array[3];

        // boolean fixed-length array field
        boolean field_boolean_fixed_array[3];

        // bytes fixed-length array field
        sequence<uint8> field_bytes_fixed_array[3];

        // float64 fixed-length array field
        double field_float64_fixed_array[3];

        // uint32 fixed-length array field
        uint32 field_uint32_fixed_array[3];

        // string fixed-length array field
        string field_string_fixed_array[3];

        // An enum field
        ExampleEnum field_enum;

        // An enum array field
        sequence<ExampleEnum> field_enum_array;

        // A nested field
        NestedMessage field_nested;

        // A nested array field
        // With
        // a
        // very
        // long
        // description
        sequence<NestedMessage> field_nested_array;
      };

      };
      "
    `);
  });

  const allIdlFiles = new Map<string, string>([
    ["Time", TIME_IDL],
    ["Duration", DURATION_IDL],
    ...Object.entries(foxgloveMessageSchemas).map(([name, schema]): [string, string] => [
      name,
      generateOmgIdl(schema),
    ]),
    ...Object.entries(foxgloveEnumSchemas).map(([name, schema]): [string, string] => [
      name,
      generateOmgIdl(schema),
    ]),
  ]);

  it.each(Object.values(foxgloveMessageSchemas))("generates parseable .idl for $name", (schema) => {
    const includePattern = /^#include "foxglove\/(.*)\.idl"$/gm;
    let idl = generateOmgIdl(schema);
    while (includePattern.test(idl)) {
      idl = idl.replace(includePattern, (_match, name: string) => {
        const file = allIdlFiles.get(name);
        if (file == undefined) {
          throw new Error(`Invalid include ${name}`);
        }
        return file;
      });
    }
    expect(() => parseIdlToMessageDefinition(idl)).not.toThrow();
  });

  it("refuses to generate enum with non-sequential values", () => {
    expect(() =>
      generateOmgIdl({
        type: "enum",
        name: "Foo",
        description: "",
        parentSchemaName: "Bar",
        protobufEnumName: "Foo",
        values: [{ name: "A", value: 1 }],
      }),
    ).toThrowErrorMatchingInlineSnapshot(
      `"Enum value Foo.A at index 0 has value 1; index and value must match for OMG IDL"`,
    );
    expect(() =>
      generateOmgIdl({
        type: "enum",
        name: "Foo",
        description: "",
        parentSchemaName: "Bar",
        protobufEnumName: "Foo",
        values: [
          { name: "A", value: 0 },
          { name: "B", value: 3 },
        ],
      }),
    ).toThrowErrorMatchingInlineSnapshot(
      `"Enum value Foo.B at index 1 has value 3; index and value must match for OMG IDL"`,
    );
  });
});
