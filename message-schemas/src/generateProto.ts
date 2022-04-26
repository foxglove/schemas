import { FoxglovePrimitive, FoxgloveSchema } from "./types";

function primitiveToProto(
  type: Exclude<FoxglovePrimitive, "integer" | "Time" | "Duration">
) {
  switch (type) {
    case "bytes":
      return "bytes";
    case "string":
      return "string";
    case "boolean":
      return "boolean";
    case "float":
      return "double";
  }
}

export function generateProto(schema: FoxgloveSchema): string {
  const imports = new Set<string>();
  const fields: string[] = [];
  let fieldNumber = 1;
  for (const field of schema.fields) {
    const lineComments: string[] = [];
    const qualifiers = [];
    if (field.array === true) {
      qualifiers.push("repeated");
    }
    switch (field.type.type) {
      case "enum":
        //FIXME
        qualifiers.push(`foxglove.${field.type.enum.name}`);
        break;
      case "nested":
        qualifiers.push(`foxglove.${field.type.schema.name}`);
        imports.add(field.type.schema.name);
        break;
      case "primitive":
        if (field.type.name === "integer") {
          qualifiers.push("int32"); // FIXME
        } else if (
          field.type.name === "Time" ||
          field.type.name === "Duration"
        ) {
          qualifiers.push(`foxglove.${field.type.name}`);
          imports.add("builtins");
        } else {
          qualifiers.push(primitiveToProto(field.type.name));
        }
        break;
    }
    //FIXME
    // if (field.arrayLength != undefined) {
    //   lineComments.push(`length ${field.arrayLength}`);
    // }
    fields.push(
      `// ${field.description}\n  ${qualifiers.join(" ")} ${
        field.name
      } = ${fieldNumber++};${
        lineComments.length > 0 ? " // " + lineComments.join(", ") : ""
      }`
    );
  }

  const outputSections = [
    `// Generated from ${schema.name} by @foxglove/message-schemas`,
    'syntax = "proto3";',

    Array.from(imports)
      .sort()
      .map((name) => `import "foxglove/${name}.proto";`)
      .join("\n"),

    `package foxglove;`,

    `message ${schema.name} {\n  ${fields.join("\n\n  ")}\n}`,
  ];

  return outputSections.join("\n\n") + "\n";
}

//FIXME: generate FileDescriptorSet?
