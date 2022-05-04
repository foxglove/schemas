import { FoxglovePrimitive, FoxgloveSchema } from "./types";

function primitiveToTypeScript(
  type: Exclude<FoxglovePrimitive, "Time" | "Duration">
) {
  switch (type) {
    case "bytes":
      return "Uint8Array";
    case "string":
      return "string";
    case "boolean":
      return "boolean";
    case "float":
    case "integer":
      return "number";
  }
}

export const TIME_TS = `export type Time = {
  sec: number;
  nsec: number;
};
`;

export const DURATION_TS = `export type Duration = {
  sec: number;
  nsec: number;
};
`;

export function generateTypeScript(schema: FoxgloveSchema): string {
  const imports = new Set<string>();

  let definition: string;
  switch (schema.type) {
    case "enum": {
      const fields = schema.values.map(({ name, value, description }) => {
        if (description != undefined) {
          return `/** ${description} */\n  ${name} = ${value},`;
        } else {
          return `${name} = ${value},`;
        }
      });
      definition = `/** ${schema.description} */\nexport enum ${
        schema.name
      } {\n  ${fields.join("\n\n  ")}\n}`;
      break;
    }

    case "message": {
      const fields = schema.fields.map((field) => {
        let fieldType: string;
        switch (field.type.type) {
          case "enum":
            fieldType = field.type.enum.name;
            imports.add(field.type.enum.name);
            break;
          case "nested":
            fieldType = field.type.schema.name;
            imports.add(field.type.schema.name);
            break;
          case "primitive":
            if (field.type.name === "Time" || field.type.name === "Duration") {
              fieldType = field.type.name;
              imports.add(field.type.name);
            } else {
              fieldType = primitiveToTypeScript(field.type.name);
            }
            break;
        }
        if (typeof field.array === "number") {
          fieldType = `[${new Array(field.array).fill(fieldType).join(", ")}]`;
        } else if (field.array != undefined) {
          fieldType = `${fieldType}[]`;
        }
        return `/** ${field.description} */\n  ${field.name}: ${fieldType};`;
      });

      definition = `/** ${schema.description} */\nexport type ${
        schema.name
      } = {\n  ${fields.join("\n\n  ")}\n};`;
      break;
    }
  }

  const outputSections = [
    `// Generated from ${schema.name} by @foxglove/message-schemas`,

    Array.from(imports)
      .sort()
      .map((name) => `import { ${name} } from "./${name}";`)
      .join("\n"),

    definition,
  ].filter(Boolean);

  return outputSections.join("\n\n") + "\n";
}
