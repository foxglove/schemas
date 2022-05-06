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
});
