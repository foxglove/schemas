import { parse as parseMessageDefinition } from "@foxglove/rosmsg";

import {
  generateRosMsgMergedSchema,
  generateRosMsgDefinition,
  generateRosMsg,
} from "./generateRos";
import { foxgloveMessageSchemas } from "./schemas";
import { exampleMessageWithoutArrayOfBytes } from "./testFixtures";

describe("generateRosMsgDefinition", () => {
  it("generates msg definition description for ROS 1", () => {
    expect(generateRosMsgDefinition(exampleMessageWithoutArrayOfBytes, { rosVersion: 1 }))
      .toMatchInlineSnapshot(`
    Object {
      "fields": Array [
        Object {
          "arrayLength": undefined,
          "description": "duration field",
          "isArray": false,
          "isComplex": false,
          "name": "field_duration",
          "type": "duration",
        },
        Object {
          "arrayLength": undefined,
          "description": "time field",
          "isArray": false,
          "isComplex": false,
          "name": "field_time",
          "type": "time",
        },
        Object {
          "arrayLength": undefined,
          "description": "boolean field",
          "isArray": false,
          "isComplex": false,
          "name": "field_boolean",
          "type": "bool",
        },
        Object {
          "arrayLength": undefined,
          "description": "bytes field",
          "isArray": true,
          "isComplex": false,
          "name": "field_bytes",
          "type": "uint8",
        },
        Object {
          "arrayLength": undefined,
          "description": "float64 field",
          "isArray": false,
          "isComplex": false,
          "name": "field_float64",
          "type": "float64",
        },
        Object {
          "arrayLength": undefined,
          "description": "uint32 field",
          "isArray": false,
          "isComplex": false,
          "name": "field_uint32",
          "type": "uint32",
        },
        Object {
          "arrayLength": undefined,
          "description": "string field",
          "isArray": false,
          "isComplex": false,
          "name": "field_string",
          "type": "string",
        },
        Object {
          "arrayLength": undefined,
          "description": "duration array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_duration_array",
          "type": "duration",
        },
        Object {
          "arrayLength": undefined,
          "description": "time array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_time_array",
          "type": "time",
        },
        Object {
          "arrayLength": undefined,
          "description": "boolean array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_boolean_array",
          "type": "bool",
        },
        Object {
          "arrayLength": undefined,
          "description": "float64 array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_float64_array",
          "type": "float64",
        },
        Object {
          "arrayLength": undefined,
          "description": "uint32 array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_uint32_array",
          "type": "uint32",
        },
        Object {
          "arrayLength": undefined,
          "description": "string array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_string_array",
          "type": "string",
        },
        Object {
          "arrayLength": 3,
          "description": "duration fixed-length array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_duration_fixed_array",
          "type": "duration",
        },
        Object {
          "arrayLength": 3,
          "description": "time fixed-length array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_time_fixed_array",
          "type": "time",
        },
        Object {
          "arrayLength": 3,
          "description": "boolean fixed-length array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_boolean_fixed_array",
          "type": "bool",
        },
        Object {
          "arrayLength": 3,
          "description": "float64 fixed-length array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_float64_fixed_array",
          "type": "float64",
        },
        Object {
          "arrayLength": 3,
          "description": "uint32 fixed-length array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_uint32_fixed_array",
          "type": "uint32",
        },
        Object {
          "arrayLength": 3,
          "description": "string fixed-length array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_string_fixed_array",
          "type": "string",
        },
        Object {
          "description": "Value A",
          "isConstant": true,
          "name": "A",
          "type": "uint8",
          "value": 1,
          "valueText": "1",
        },
        Object {
          "description": "Value B",
          "isConstant": true,
          "name": "B",
          "type": "uint8",
          "value": 2,
          "valueText": "2",
        },
        Object {
          "arrayLength": undefined,
          "description": "An enum field",
          "isArray": false,
          "isComplex": false,
          "name": "field_enum",
          "type": "uint8",
        },
        Object {
          "arrayLength": undefined,
          "description": "An enum array field",
          "isArray": true,
          "isComplex": false,
          "name": "field_enum_array",
          "type": "uint8",
        },
        Object {
          "arrayLength": undefined,
          "description": "A nested field",
          "isArray": false,
          "isComplex": true,
          "name": "field_nested",
          "type": "foxglove_msgs/NestedMessage",
        },
        Object {
          "arrayLength": undefined,
          "description": "A nested array field
    With
    a
    very
    long
    description",
          "isArray": true,
          "isComplex": true,
          "name": "field_nested_array",
          "type": "foxglove_msgs/NestedMessage",
        },
      ],
      "originalName": "ExampleMessage",
      "rosFullInterfaceName": "foxglove_msgs/ExampleMessage",
      "rosMsgInterfaceName": "foxglove_msgs/ExampleMessage",
    }
  `);
  });
  it("generates msg definition description for ROS 2", () => {
    expect(generateRosMsgDefinition(exampleMessageWithoutArrayOfBytes, { rosVersion: 2 }))
      .toMatchInlineSnapshot(`
  Object {
    "fields": Array [
      Object {
        "arrayLength": undefined,
        "description": "duration field",
        "isArray": false,
        "isComplex": false,
        "name": "field_duration",
        "type": "builtin_interfaces/Duration",
      },
      Object {
        "arrayLength": undefined,
        "description": "time field",
        "isArray": false,
        "isComplex": false,
        "name": "field_time",
        "type": "builtin_interfaces/Time",
      },
      Object {
        "arrayLength": undefined,
        "description": "boolean field",
        "isArray": false,
        "isComplex": false,
        "name": "field_boolean",
        "type": "bool",
      },
      Object {
        "arrayLength": undefined,
        "description": "bytes field",
        "isArray": true,
        "isComplex": false,
        "name": "field_bytes",
        "type": "uint8",
      },
      Object {
        "arrayLength": undefined,
        "description": "float64 field",
        "isArray": false,
        "isComplex": false,
        "name": "field_float64",
        "type": "float64",
      },
      Object {
        "arrayLength": undefined,
        "description": "uint32 field",
        "isArray": false,
        "isComplex": false,
        "name": "field_uint32",
        "type": "uint32",
      },
      Object {
        "arrayLength": undefined,
        "description": "string field",
        "isArray": false,
        "isComplex": false,
        "name": "field_string",
        "type": "string",
      },
      Object {
        "arrayLength": undefined,
        "description": "duration array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_duration_array",
        "type": "builtin_interfaces/Duration",
      },
      Object {
        "arrayLength": undefined,
        "description": "time array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_time_array",
        "type": "builtin_interfaces/Time",
      },
      Object {
        "arrayLength": undefined,
        "description": "boolean array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_boolean_array",
        "type": "bool",
      },
      Object {
        "arrayLength": undefined,
        "description": "float64 array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_float64_array",
        "type": "float64",
      },
      Object {
        "arrayLength": undefined,
        "description": "uint32 array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_uint32_array",
        "type": "uint32",
      },
      Object {
        "arrayLength": undefined,
        "description": "string array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_string_array",
        "type": "string",
      },
      Object {
        "arrayLength": 3,
        "description": "duration fixed-length array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_duration_fixed_array",
        "type": "builtin_interfaces/Duration",
      },
      Object {
        "arrayLength": 3,
        "description": "time fixed-length array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_time_fixed_array",
        "type": "builtin_interfaces/Time",
      },
      Object {
        "arrayLength": 3,
        "description": "boolean fixed-length array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_boolean_fixed_array",
        "type": "bool",
      },
      Object {
        "arrayLength": 3,
        "description": "float64 fixed-length array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_float64_fixed_array",
        "type": "float64",
      },
      Object {
        "arrayLength": 3,
        "description": "uint32 fixed-length array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_uint32_fixed_array",
        "type": "uint32",
      },
      Object {
        "arrayLength": 3,
        "description": "string fixed-length array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_string_fixed_array",
        "type": "string",
      },
      Object {
        "description": "Value A",
        "isConstant": true,
        "name": "A",
        "type": "uint8",
        "value": 1,
        "valueText": "1",
      },
      Object {
        "description": "Value B",
        "isConstant": true,
        "name": "B",
        "type": "uint8",
        "value": 2,
        "valueText": "2",
      },
      Object {
        "arrayLength": undefined,
        "description": "An enum field",
        "isArray": false,
        "isComplex": false,
        "name": "field_enum",
        "type": "uint8",
      },
      Object {
        "arrayLength": undefined,
        "description": "An enum array field",
        "isArray": true,
        "isComplex": false,
        "name": "field_enum_array",
        "type": "uint8",
      },
      Object {
        "arrayLength": undefined,
        "description": "A nested field",
        "isArray": false,
        "isComplex": true,
        "name": "field_nested",
        "type": "foxglove_msgs/NestedMessage",
      },
      Object {
        "arrayLength": undefined,
        "description": "A nested array field
  With
  a
  very
  long
  description",
        "isArray": true,
        "isComplex": true,
        "name": "field_nested_array",
        "type": "foxglove_msgs/NestedMessage",
      },
    ],
    "originalName": "ExampleMessage",
    "rosFullInterfaceName": "foxglove_msgs/msg/ExampleMessage",
    "rosMsgInterfaceName": "foxglove_msgs/ExampleMessage",
  }
`);
  });
});

describe("generateRosMsg", () => {
  it("generates msg file for ROS 1", () => {
    expect(
      generateRosMsg(
        generateRosMsgDefinition(exampleMessageWithoutArrayOfBytes, {
          rosVersion: 1,
        }),
      ),
    ).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/schemas

      # duration field
      duration field_duration

      # time field
      time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float64 field
      float64 field_float64

      # uint32 field
      uint32 field_uint32

      # string field
      string field_string

      # duration array field
      duration[] field_duration_array

      # time array field
      time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float64 array field
      float64[] field_float64_array

      # uint32 array field
      uint32[] field_uint32_array

      # string array field
      string[] field_string_array

      # duration fixed-length array field
      duration[3] field_duration_fixed_array

      # time fixed-length array field
      time[3] field_time_fixed_array

      # boolean fixed-length array field
      bool[3] field_boolean_fixed_array

      # float64 fixed-length array field
      float64[3] field_float64_fixed_array

      # uint32 fixed-length array field
      uint32[3] field_uint32_fixed_array

      # string fixed-length array field
      string[3] field_string_fixed_array

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
      # With
      # a
      # very
      # long
      # description
      foxglove_msgs/NestedMessage[] field_nested_array
      "
    `);
  });

  it("generates msg file for ROS 2", () => {
    expect(
      generateRosMsg(
        generateRosMsgDefinition(exampleMessageWithoutArrayOfBytes, {
          rosVersion: 2,
        }),
      ),
    ).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/schemas

      # duration field
      builtin_interfaces/Duration field_duration

      # time field
      builtin_interfaces/Time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float64 field
      float64 field_float64

      # uint32 field
      uint32 field_uint32

      # string field
      string field_string

      # duration array field
      builtin_interfaces/Duration[] field_duration_array

      # time array field
      builtin_interfaces/Time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float64 array field
      float64[] field_float64_array

      # uint32 array field
      uint32[] field_uint32_array

      # string array field
      string[] field_string_array

      # duration fixed-length array field
      builtin_interfaces/Duration[3] field_duration_fixed_array

      # time fixed-length array field
      builtin_interfaces/Time[3] field_time_fixed_array

      # boolean fixed-length array field
      bool[3] field_boolean_fixed_array

      # float64 fixed-length array field
      float64[3] field_float64_fixed_array

      # uint32 fixed-length array field
      uint32[3] field_uint32_fixed_array

      # string fixed-length array field
      string[3] field_string_fixed_array

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
      # With
      # a
      # very
      # long
      # description
      foxglove_msgs/NestedMessage[] field_nested_array
      "
    `);
  });
});

describe("generateRosMsgMergedSchema", () => {
  it("generates merged schema for ROS 1", () => {
    const mergedSchema = generateRosMsgMergedSchema(exampleMessageWithoutArrayOfBytes, {
      rosVersion: 1,
    });
    expect(mergedSchema).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/schemas

      # duration field
      duration field_duration

      # time field
      time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float64 field
      float64 field_float64

      # uint32 field
      uint32 field_uint32

      # string field
      string field_string

      # duration array field
      duration[] field_duration_array

      # time array field
      time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float64 array field
      float64[] field_float64_array

      # uint32 array field
      uint32[] field_uint32_array

      # string array field
      string[] field_string_array

      # duration fixed-length array field
      duration[3] field_duration_fixed_array

      # time fixed-length array field
      time[3] field_time_fixed_array

      # boolean fixed-length array field
      bool[3] field_boolean_fixed_array

      # float64 fixed-length array field
      float64[3] field_float64_fixed_array

      # uint32 fixed-length array field
      uint32[3] field_uint32_fixed_array

      # string fixed-length array field
      string[3] field_string_fixed_array

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
      # With
      # a
      # very
      # long
      # description
      foxglove_msgs/NestedMessage[] field_nested_array
      ================================================================================
      MSG: foxglove_msgs/NestedMessage
      # Generated from NestedMessage by @foxglove/schemas

      # An enum field
      uint32 field_enum
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
              "name": "field_float64",
              "type": "float64",
            },
            Object {
              "isArray": false,
              "isComplex": false,
              "name": "field_uint32",
              "type": "uint32",
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
              "name": "field_float64_array",
              "type": "float64",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_uint32_array",
              "type": "uint32",
            },
            Object {
              "isArray": true,
              "isComplex": false,
              "name": "field_string_array",
              "type": "string",
            },
            Object {
              "arrayLength": 3,
              "isArray": true,
              "isComplex": false,
              "name": "field_duration_fixed_array",
              "type": "duration",
            },
            Object {
              "arrayLength": 3,
              "isArray": true,
              "isComplex": false,
              "name": "field_time_fixed_array",
              "type": "time",
            },
            Object {
              "arrayLength": 3,
              "isArray": true,
              "isComplex": false,
              "name": "field_boolean_fixed_array",
              "type": "bool",
            },
            Object {
              "arrayLength": 3,
              "isArray": true,
              "isComplex": false,
              "name": "field_float64_fixed_array",
              "type": "float64",
            },
            Object {
              "arrayLength": 3,
              "isArray": true,
              "isComplex": false,
              "name": "field_uint32_fixed_array",
              "type": "uint32",
            },
            Object {
              "arrayLength": 3,
              "isArray": true,
              "isComplex": false,
              "name": "field_string_fixed_array",
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
              "type": "uint32",
            },
          ],
          "name": "foxglove_msgs/NestedMessage",
        },
      ]
    `);
  });

  it("generates merged schema for ROS 2", () => {
    const mergedSchema = generateRosMsgMergedSchema(exampleMessageWithoutArrayOfBytes, {
      rosVersion: 2,
    });
    expect(mergedSchema).toMatchInlineSnapshot(`
      "# Generated from ExampleMessage by @foxglove/schemas

      # duration field
      builtin_interfaces/Duration field_duration

      # time field
      builtin_interfaces/Time field_time

      # boolean field
      bool field_boolean

      # bytes field
      uint8[] field_bytes

      # float64 field
      float64 field_float64

      # uint32 field
      uint32 field_uint32

      # string field
      string field_string

      # duration array field
      builtin_interfaces/Duration[] field_duration_array

      # time array field
      builtin_interfaces/Time[] field_time_array

      # boolean array field
      bool[] field_boolean_array

      # float64 array field
      float64[] field_float64_array

      # uint32 array field
      uint32[] field_uint32_array

      # string array field
      string[] field_string_array

      # duration fixed-length array field
      builtin_interfaces/Duration[3] field_duration_fixed_array

      # time fixed-length array field
      builtin_interfaces/Time[3] field_time_fixed_array

      # boolean fixed-length array field
      bool[3] field_boolean_fixed_array

      # float64 fixed-length array field
      float64[3] field_float64_fixed_array

      # uint32 fixed-length array field
      uint32[3] field_uint32_fixed_array

      # string fixed-length array field
      string[3] field_string_fixed_array

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
      # With
      # a
      # very
      # long
      # description
      foxglove_msgs/NestedMessage[] field_nested_array
      ================================================================================
      MSG: foxglove_msgs/NestedMessage
      # Generated from NestedMessage by @foxglove/schemas

      # An enum field
      uint32 field_enum
      "
    `);
    expect(parseMessageDefinition(mergedSchema, { ros2: true })).toMatchInlineSnapshot(`
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
              "name": "field_float64",
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
              "name": "field_uint32",
              "type": "uint32",
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
              "name": "field_float64_array",
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
              "name": "field_uint32_array",
              "type": "uint32",
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
              "arrayLength": 3,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_duration_fixed_array",
              "type": "duration",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": 3,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_time_fixed_array",
              "type": "time",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": 3,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_boolean_fixed_array",
              "type": "bool",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": 3,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_float64_fixed_array",
              "type": "float64",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": 3,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_uint32_fixed_array",
              "type": "uint32",
              "upperBound": undefined,
              "value": undefined,
              "valueText": undefined,
            },
            Object {
              "arrayLength": 3,
              "arrayUpperBound": undefined,
              "defaultValue": undefined,
              "isArray": true,
              "isComplex": false,
              "isConstant": undefined,
              "name": "field_string_fixed_array",
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
              "type": "uint32",
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

  it.each(Object.values(foxgloveMessageSchemas))("generates parseable merged schemas", (schema) => {
    expect(() =>
      parseMessageDefinition(generateRosMsgMergedSchema(schema, { rosVersion: 1 })),
    ).not.toThrow();

    expect(() =>
      parseMessageDefinition(generateRosMsgMergedSchema(schema, { rosVersion: 2 }), { ros2: true }),
    ).not.toThrow();
  });
});
