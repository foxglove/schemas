import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "./schemas";

describe("schemas", () => {
  it("has matching names", () => {
    for (const [key, value] of Object.entries(foxgloveMessageSchemas)) {
      expect(key).toEqual(value.name);
    }
    for (const [key, value] of Object.entries(foxgloveEnumSchemas)) {
      expect(key).toEqual(value.name);
      expect(value.protobufParentMessageName in foxgloveMessageSchemas).toBe(true);
    }
  });

  it("has valid descriptions", () => {
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      // Multi-line descriptions are supported for fields, but not currently for schemas
      expect(schema.description.includes("\n")).toBe(false);
      for (const field of schema.fields) {
        expect(field.description.includes("*/")).toBe(false);
      }
    }
  });
});
