import { FoxglovePrimitive, FoxgloveSchema } from "./types";

function primitiveToTypeScript(type: Exclude<FoxglovePrimitive, "time" | "duration">) {
  switch (type) {
    case "bytes":
      return "Uint8Array";
    case "string":
      return "string";
    case "boolean":
      return "boolean";
    case "float64":
    case "uint32":
      return "number";
  }
}

function primitiveToTypedArray(type: FoxglovePrimitive) {
  switch (type) {
    case "time":
    case "duration":
    case "bytes":
    case "string":
    case "boolean":
      return [];
    case "float64":
      return ["Float32Array", "Float64Array"];
    case "uint32":
      return ["Uint32Array"];
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

export type GenerateTypeScriptOptions = {
  /**
   * Include TypedArray types on numeric array fields, e.g. `number[] | Uint32Array`. Used by
   * Foxglove to accept TypedArrays for user script outputs.
   */
  includeTypedArrays?: boolean;
};

export function generateTypeScript(
  schema: FoxgloveSchema,
  options: GenerateTypeScriptOptions = {},
): string {
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
      definition = `/** ${schema.description} */\nexport enum ${schema.name} {\n  ${fields.join(
        "\n\n  ",
      )}\n}`;
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
            if (field.type.name === "time") {
              fieldType = "Time";
              imports.add("Time");
            } else if (field.type.name === "duration") {
              fieldType = "Duration";
              imports.add("Duration");
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
        if (
          field.array != undefined &&
          options.includeTypedArrays === true &&
          field.type.type === "primitive"
        ) {
          fieldType = [fieldType, ...primitiveToTypedArray(field.type.name)].join(" | ");
        }
        let comment: string;
        const descriptionLines = field.description.trim().split("\n");
        if (descriptionLines.length === 1) {
          comment = `/** ${field.description} */`;
        } else {
          comment = `/**\n  ${descriptionLines.map((line) => ` * ${line}`).join("\n  ")}\n   */`;
        }
        return `${comment}\n  ${field.name}: ${fieldType};`;
      });

      definition = `/** ${schema.description} */\nexport type ${schema.name} = {\n  ${fields.join(
        "\n\n  ",
      )}\n};`;
      break;
    }
  }

  const outputSections = [
    `// Generated by https://github.com/foxglove/foxglove-sdk\n// Options: ${JSON.stringify(options)}`,

    Array.from(imports)
      .sort()
      .map((name) => `import { ${name} } from "./${name}";`)
      .join("\n"),

    definition,
  ].filter(Boolean);

  return outputSections.join("\n\n") + "\n";
}
