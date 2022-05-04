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
      return "bool";
    case "float":
      return "double";
  }
}

export const TIME_PROTO = `syntax = "proto3";

package foxglove;

message Time {
  fixed32 sec = 1;
  fixed32 nsec = 2;
}
`;

export const DURATION_PROTO = `syntax = "proto3";

package foxglove;

message Duration {
  fixed32 sec = 1;
  fixed32 nsec = 2;
}
`;

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
      definition = `// ${schema.description}\nenum ${
        schema.name
      } {\n  ${fields.join("\n\n  ")}\n}`;
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
            imports.add(field.type.enum.name);
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
              imports.add(field.type.name);
            } else {
              qualifiers.push(primitiveToProto(field.type.name));
            }
            break;
        }
        return `// ${field.description}\n  ${qualifiers.join(" ")} ${
          field.name
        } = ${fieldNumber++};${
          lineComments.length > 0 ? " // " + lineComments.join(", ") : ""
        }`;
      });

      definition = `// ${schema.description}\nmessage ${
        schema.name
      } {\n  ${fields.join("\n\n  ")}\n}`;
      break;
    }
  }

  const outputSections = [
    `// Generated from ${schema.name} by @foxglove/message-schemas`,
    'syntax = "proto3";',

    Array.from(imports)
      .sort()
      .map((name) => `import "foxglove/${name}.proto";`)
      .join("\n"),

    `package foxglove;`,

    definition,
  ].filter(Boolean);

  return outputSections.join("\n\n") + "\n";
}
