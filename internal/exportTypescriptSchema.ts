import { DURATION_TS, TIME_TS, generateTypeScript } from "./generateTypeScript";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "./schemas";

/**
 * Export typescript schema as source, keyed by the schema name.
 *
 * @returns a record of schema name => schema source.
 */
export function exportTypescriptSchema(): Record<string, string> {
  const schemas: Record<string, string> = {};

  for (const schema of Object.values(foxgloveMessageSchemas)) {
    schemas[schema.name] = generateTypeScript(schema);
  }

  for (const schema of Object.values(foxgloveEnumSchemas)) {
    schemas[schema.name] = generateTypeScript(schema);
  }

  schemas["Duration"] = DURATION_TS;
  schemas["Time"] = TIME_TS;

  const allSchemaNames = [
    ...Object.values(foxgloveMessageSchemas),
    ...Object.values(foxgloveEnumSchemas),
  ].sort((a, b) => a.name.localeCompare(b.name));
  let indexTS = "";
  for (const schema of allSchemaNames) {
    indexTS += `export * from "./${schema.name}";\n`;
  }
  schemas["index"] = indexTS;

  return schemas;
}
