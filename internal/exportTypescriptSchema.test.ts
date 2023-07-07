import { exportTypescriptSchema } from "./exportTypescriptSchema";

describe("exportTypescriptSchema", () => {
  it("exports schemas", () => {
    const schemas = exportTypescriptSchema();
    expect(schemas["ArrowPrimitive"]).not.toBeNull();
  });
});
