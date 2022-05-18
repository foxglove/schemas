import { generateTypeScript } from "./generateTypeScript";
import { exampleEnum, exampleMessage } from "./testFixtures";

describe("generateTypeScript", () => {
  it("generates .d.ts files", () => {
    expect(generateTypeScript(exampleEnum)).toMatchInlineSnapshot(`
      "// Generated from ExampleEnum by @foxglove/schemas

      /** An example enum */
      export enum ExampleEnum {
        /** Value A */
        A = 1,

        /** Value B */
        B = 2,
      }
      "
    `);
    expect(generateTypeScript(exampleMessage)).toMatchInlineSnapshot(`
      "// Generated from ExampleMessage by @foxglove/schemas

      import { Duration } from \\"./Duration\\";
      import { ExampleEnum } from \\"./ExampleEnum\\";
      import { NestedMessage } from \\"./NestedMessage\\";
      import { Time } from \\"./Time\\";

      /** An example type */
      export type ExampleMessage = {
        /** duration field */
        field_duration: Duration;

        /** time field */
        field_time: Time;

        /** boolean field */
        field_boolean: boolean;

        /** bytes field */
        field_bytes: Uint8Array;

        /** float64 field */
        field_float64: number;

        /** uint32 field */
        field_uint32: number;

        /** string field */
        field_string: string;

        /** duration array field */
        field_duration_array: Duration[];

        /** time array field */
        field_time_array: Time[];

        /** boolean array field */
        field_boolean_array: boolean[];

        /** bytes array field */
        field_bytes_array: Uint8Array[];

        /** float64 array field */
        field_float64_array: number[];

        /** uint32 array field */
        field_uint32_array: number[];

        /** string array field */
        field_string_array: string[];

        /** duration fixed-length array field */
        field_duration_fixed_array: [Duration, Duration, Duration];

        /** time fixed-length array field */
        field_time_fixed_array: [Time, Time, Time];

        /** boolean fixed-length array field */
        field_boolean_fixed_array: [boolean, boolean, boolean];

        /** bytes fixed-length array field */
        field_bytes_fixed_array: [Uint8Array, Uint8Array, Uint8Array];

        /** float64 fixed-length array field */
        field_float64_fixed_array: [number, number, number];

        /** uint32 fixed-length array field */
        field_uint32_fixed_array: [number, number, number];

        /** string fixed-length array field */
        field_string_fixed_array: [string, string, string];

        /** An enum field */
        field_enum: ExampleEnum;

        /** An enum array field */
        field_enum_array: ExampleEnum[];

        /** A nested field */
        field_nested: NestedMessage;

        /**
         * A nested array field
         * With
         * a
         * very
         * long
         * description
         */
        field_nested_array: NestedMessage[];
      };
      "
    `);
  });

  it("generates parseable .ts files", async () => {
    await expect(import("../schemas/typescript")).resolves.not.toThrow();
  });
});
