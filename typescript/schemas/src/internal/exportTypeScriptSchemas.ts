import {
  DURATION_TS,
  GenerateTypeScriptOptions,
  TIME_TS,
  generateTypeScript,
} from "./generateTypeScript";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "./schemas";

/**
 * Export schemas as TypeScript source code, keyed by the file base name (without `.ts` suffix).
 *
 * @returns a map of file base name => schema source.
 */
export function exportTypeScriptSchemas(
  options: GenerateTypeScriptOptions = {},
): Map<string, string> {
  const schemas = new Map<string, string>();

  for (const schema of Object.values(foxgloveMessageSchemas)) {
    schemas.set(schema.name, generateTypeScript(schema, options));
  }

  for (const schema of Object.values(foxgloveEnumSchemas)) {
    schemas.set(schema.name, generateTypeScript(schema, options));
  }

  schemas.set("Duration", DURATION_TS);
  schemas.set("Time", TIME_TS);

  const allSchemaNames = [
    ...Object.values(foxgloveMessageSchemas),
    ...Object.values(foxgloveEnumSchemas),
  ].sort((a, b) => a.name.localeCompare(b.name));
  let indexTS = "";
  for (const schema of allSchemaNames) {
    indexTS += `export * from "./${schema.name}";\n`;
  }
  schemas.set("index", indexTS);

  return schemas;
}
