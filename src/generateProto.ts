import { FoxgloveEnumSchema, FoxgloveMessageSchema, FoxglovePrimitive } from "./types";

function primitiveToProto(type: Exclude<FoxglovePrimitive, "time" | "duration">) {
  switch (type) {
    case "uint32":
      return "fixed32";
    case "bytes":
      return "bytes";
    case "string":
      return "string";
    case "boolean":
      return "bool";
    case "float64":
      return "double";
  }
}

export function generateProto(
  schema: FoxgloveMessageSchema,
  nestedEnums: FoxgloveEnumSchema[],
): string {
  const enumDefinitions: string[] = [];
  for (const enumSchema of nestedEnums) {
    const fields = enumSchema.values.map(({ name, value, description }) => {
      if (description != undefined) {
        return `// ${description}\n    ${name} = ${value};`;
      } else {
        return `${name} = ${value};`;
      }
    });
    enumDefinitions.push(
      `  // ${enumSchema.description}\n  enum ${enumSchema.protobufEnumName} {\n    ${fields.join(
        "\n\n    ",
      )}\n  }\n`,
    );
  }

  let fieldNumber = 1;
  const imports = new Set<string>();
  const fields = schema.fields.map((field) => {
    const lineComments: string[] = [];
    const qualifiers: string[] = [];
    if (field.array != undefined) {
      qualifiers.push("repeated");
    }
    if (typeof field.array === "number") {
      lineComments.push(`length ${field.array}`);
    }
    switch (field.type.type) {
      case "enum":
        qualifiers.push(field.type.enum.protobufEnumName);
        break;
      case "nested":
        qualifiers.push(`foxglove.${field.type.schema.name}`);
        imports.add(`foxglove/${field.type.schema.name}`);
        break;
      case "primitive":
        if (field.type.name === "time") {
          qualifiers.push("google.protobuf.Timestamp");
          imports.add(`google/protobuf/timestamp`);
        } else if (field.type.name === "duration") {
          qualifiers.push("google.protobuf.Duration");
          imports.add(`google/protobuf/duration`);
        } else {
          qualifiers.push(primitiveToProto(field.type.name));
        }
        break;
    }
    return `${field.description
      .trim()
      .split("\n")
      .map((line) => `  // ${line}\n`)
      .join("")}  ${qualifiers.join(" ")} ${field.name} = ${fieldNumber++};${
      lineComments.length > 0 ? " // " + lineComments.join(", ") : ""
    }`;
  });

  const definition = `// ${schema.description}\nmessage ${schema.name} {\n${enumDefinitions.join(
    "\n\n",
  )}${fields.join("\n\n")}\n}`;

  const outputSections = [
    `// Generated from ${schema.name} by @foxglove/schemas`,
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
