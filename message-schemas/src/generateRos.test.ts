import { parse as parseMessageDefinition } from "@foxglove/rosmsg";

import {
  generateRosMsgMergedSchema,
  generateRosMsgDefinition,
  generateRosMsg,
} from "./generateRos";
import { foxgloveMessageSchemas } from "./schemas";
import { exampleMessageWithoutArrayOfBytes } from "./testFixtures";

describe("generateRosMsgFiles", () => {
  it("generates msg file for ROS 1", () => {
    expect(
      generateRosMsg(
        generateRosMsgDefinition(exampleMessageWithoutArrayOfBytes, {
          rosVersion: 1,
        })
      )
    ).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/message-schemas

      # Duration field
      duration field_duration

      # Time field
      time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float field
      float64 field_float

      # integer field
      int32 field_integer

      # string field
      string field_string

      # Duration array field
      duration[] field_duration_array

      # Time array field
      time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float array field
      float64[] field_float_array

      # integer array field
      int32[] field_integer_array

      # string array field
      string[] field_string_array

      # Value A
      uint8 A=1

      # Value B
      uint8 B=2

      # An enum field
      uint8 field_enum

      # An enum array field
      uint8[] field_enum_array

      # A nested field
      foxglove_msgs/NestedMessage field_nested

      # A nested array field
      foxglove_msgs/NestedMessage[] field_nested_array
      "
    `);
  });

  it("generates msg file for ROS 2", () => {
    expect(
      generateRosMsg(
        generateRosMsgDefinition(exampleMessageWithoutArrayOfBytes, {
          rosVersion: 2,
        })
      )
    ).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/message-schemas

      # Duration field
      builtin_interfaces/Duration field_duration

      # Time field
      builtin_interfaces/Time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float field
      float64 field_float

      # integer field
      int32 field_integer

      # string field
      string field_string

      # Duration array field
      builtin_interfaces/Duration[] field_duration_array

      # Time array field
      builtin_interfaces/Time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float array field
      float64[] field_float_array

      # integer array field
      int32[] field_integer_array

      # string array field
      string[] field_string_array

      # Value A
      uint8 A=1

      # Value B
      uint8 B=2

      # An enum field
      uint8 field_enum

      # An enum array field
      uint8[] field_enum_array

      # A nested field
      foxglove_msgs/NestedMessage field_nested

      # A nested array field
      foxglove_msgs/NestedMessage[] field_nested_array
      "
    `);
  });
});

describe("generateRosMsgMergedSchema", () => {
  it("generates merged schema for ROS 1", () => {
    const mergedSchema = generateRosMsgMergedSchema(
      exampleMessageWithoutArrayOfBytes,
      { rosVersion: 1 }
    );
    expect(mergedSchema).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/message-schemas

      # Duration field
      duration field_duration

      # Time field
      time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float field
      float64 field_float

      # integer field
      int32 field_integer

      # string field
      string field_string

      # Duration array field
      duration[] field_duration_array

      # Time array field
      time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float array field
      float64[] field_float_array

      # integer array field
      int32[] field_integer_array

      # string array field
      string[] field_string_array

      # Value A
      uint8 A=1

      # Value B
      uint8 B=2

      # An enum field
      uint8 field_enum

      # An enum array field
      uint8[] field_enum_array

      # A nested field
      foxglove_msgs/NestedMessage field_nested

      # A nested array field
      foxglove_msgs/NestedMessage[] field_nested_array
      ================================================================================
      MSG: foxglove_msgs/NestedMessage
      # Generated from NestedMessage by @foxglove/message-schemas

      # An enum field
      int32 field_enum
      "
    `);
    expect(parseMessageDefinition(mergedSchema)).toMatchInlineSnapshot(`
      Array [
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_duration",
              "type": "duration",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_time",
              "type": "time",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_boolean",
              "type": "bool",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_bytes",
              "type": "uint8",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_float",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_integer",
              "type": "int32",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_string",
              "type": "string",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_duration_array",
              "type": "duration",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_time_array",
              "type": "time",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_boolean_array",
              "type": "bool",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_float_array",
              "type": "float64",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_integer_array",
              "type": "int32",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_string_array",
              "type": "string",
            },
            Object {
              "isConstant": true,
              "name": "A",
              "type": "uint8",
              "value": 1,
              "valueText": "1",
            },
            Object {
              "isConstant": true,
              "name": "B",
              "type": "uint8",
              "value": 2,
              "valueText": "2",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_enum",
              "type": "uint8",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_enum_array",
              "type": "uint8",
            },
            Object {
              "isArray": false,
              "isComplex": true,
              "name": "field_nested",
              "type": "foxglove_msgs/NestedMessage",
            },
            Object {
              "isArray": true,
              "isComplex": true,
              "name": "field_nested_array",
              "type": "foxglove_msgs/NestedMessage",
            },
          ],
          "name": undefined,
        },
        Object {
          "definitions": Array [
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_enum",
              "type": "int32",
            },
          ],
          "name": "foxglove_msgs/NestedMessage",
        },
      ]
    `);
  });

  it("generates merged schema for ROS 2", () => {
    const mergedSchema = generateRosMsgMergedSchema(
      exampleMessageWithoutArrayOfBytes,
      { rosVersion: 2 }
    );
    expect(mergedSchema).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/message-schemas

      # Duration field
      builtin_interfaces/Duration field_duration

      # Time field
      builtin_interfaces/Time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float field
      float64 field_float

      # integer field
      int32 field_integer

      # string field
      string field_string

      # Duration array field
      builtin_interfaces/Duration[] field_duration_array

      # Time array field
      builtin_interfaces/Time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float array field
      float64[] field_float_array

      # integer array field
      int32[] field_integer_array

      # string array field
      string[] field_string_array

      # Value A
      uint8 A=1

      # Value B
      uint8 B=2

      # An enum field
      uint8 field_enum

      # An enum array field
      uint8[] field_enum_array

      # A nested field
      foxglove_msgs/NestedMessage field_nested

      # A nested array field
      foxglove_msgs/NestedMessage[] field_nested_array
      ================================================================================
      MSG: foxglove_msgs/NestedMessage
      # Generated from NestedMessage by @foxglove/message-schemas

      # An enum field
      int32 field_enum
      "
    `);
    expect(parseMessageDefinition(mergedSchema, { ros2: true }))
      .toMatchInlineSnapshot(`
      Array [
        Object {
          "definitions": Array [
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_duration",
              "type": "duration",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_time",
              "type": "time",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_boolean",
              "type": "bool",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_bytes",
              "type": "uint8",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_float",
              "type": "float64",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_integer",
              "type": "int32",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_string",
              "type": "string",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_duration_array",
              "type": "duration",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_time_array",
              "type": "time",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_boolean_array",
              "type": "bool",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_float_array",
              "type": "float64",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_integer_array",
              "type": "int32",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_string_array",
              "type": "string",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": undefined,
              "isComplex": undefined,
              "isConstant": true,
              "name": "A",
              "type": "uint8",
              "upperBound": undefined,
              "value": 1,
              "valueText": "1",
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": undefined,
              "isComplex": undefined,
              "isConstant": true,
              "name": "B",
              "type": "uint8",
              "upperBound": undefined,
              "value": 2,
              "valueText": "2",
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_enum",
              "type": "uint8",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_enum_array",
              "type": "uint8",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": true,
              "isConstant": undefined,
              "name": "field_nested",
              "type": "foxglove_msgs/NestedMessage",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": true,
              "isConstant": undefined,
              "name": "field_nested_array",
              "type": "foxglove_msgs/NestedMessage",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
          ],
          "name": undefined,
        },
        Object {
          "definitions": Array [
            Object {
              "arrayLength": undefined,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": false,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_enum",
              "type": "int32",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
          ],
          "name": "foxglove_msgs/NestedMessage",
        },
      ]
    `);
  });

  it.each(Object.values(foxgloveMessageSchemas))(
    "generates parseable merged schemas",
    (schema) => {
      expect(() =>
        parseMessageDefinition(
          generateRosMsgMergedSchema(schema, { rosVersion: 1 })
        )
      ).not.toThrow();

      expect(() =>
        parseMessageDefinition(
          generateRosMsgMergedSchema(schema, { rosVersion: 2 }),
          { ros2: true }
        )
      ).not.toThrow();
    }
  );
});
