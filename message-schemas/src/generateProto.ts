import { FoxglovePrimitive, FoxgloveSchema } from "./types";

function primitiveToProto(type: Exclude<FoxglovePrimitive, "uint32" | "Time" | "Duration">) {
  switch (type) {
    case "bytes":
      return "bytes";
    case "string":
      return "string";
    case "boolean":
      return "bool";
    case "float":
      return "double";
  }
}

export function generateProto(schema: FoxgloveSchema): string {
  const imports = new Set<string>();
  let fieldNumber = 1;

  let definition: string;
  switch (schema.type) {
    case "enum": {
      const fields = schema.values.map(({ name, value, description }) => {
        if (description != undefined) {
          return `// ${description}\n  ${name} = ${value};`;
        } else {
          return `${name} = ${value};`;
        }
      });
      definition = `// ${schema.description}\nenum ${schema.name} {\n  ${fields.join("\n\n  ")}\n}`;
      break;
    }

    case "message": {
      const fields = schema.fields.map((field) => {
        const lineComments: string[] = [];
        const qualifiers = [];
        if (field.array != undefined) {
          qualifiers.push("repeated");
        }
        if (typeof field.array === "number") {
          lineComments.push(`length ${field.array}`);
        }
        switch (field.type.type) {
          case "enum":
            qualifiers.push(`foxglove.${field.type.enum.name}`);
            imports.add(`foxglove/${field.type.enum.name}`);
            break;
          case "nested":
            qualifiers.push(`foxglove.${field.type.schema.name}`);
            imports.add(`foxglove/${field.type.schema.name}`);
            break;
          case "primitive":
            if (field.type.name === "uint32") {
              qualifiers.push("fixed32");
            } else if (field.type.name === "Time") {
              qualifiers.push("google.protobuf.Timestamp");
              imports.add(`google/protobuf/timestamp`);
            } else if (field.type.name === "Duration") {
              qualifiers.push("google.protobuf.Duration");
              imports.add(`google/protobuf/duration`);
            } else {
              qualifiers.push(primitiveToProto(field.type.name));
            }
            break;
        }
        return `// ${field.description}\n  ${qualifiers.join(" ")} ${
          field.name
        } = ${fieldNumber++};${lineComments.length > 0 ? " // " + lineComments.join(", ") : ""}`;
      });

      definition = `// ${schema.description}\nmessage ${schema.name} {\n  ${fields.join(
        "\n\n  ",
      )}\n}`;
      break;
    }
  }

  const outputSections = [
    `// Generated from ${schema.name} by @foxglove/message-schemas`,
    'syntax = "proto3";',

    Array.from(imports)
      .sort()
      .map((name) => `import "${name}.proto";`)
      .join("\n"),

    `package foxglove;`,

    definition,
  ].filter(Boolean);

  return outputSections.join("\n\n") + "\n";
}
