import { exportTypescriptSchemata } from "./exportTypescriptSchemata";

describe("exportTypescriptSchemata", () => {
  it("exports schemata", () => {
    const schemas = exportTypescriptSchemata();
    expect(schemas).toMatchSnapshot();
  });
});
