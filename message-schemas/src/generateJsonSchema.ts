import { FoxglovePrimitive, FoxgloveSchema } from "./types";

function primitiveToJsonSchema(type: Exclude<FoxglovePrimitive, "bytes">) {
  //FIXME
  switch (type) {
    case "string":
      return { type: "string" };
    case "boolean":
      return { type: "boolean" };
    case "float":
    case "integer":
      return { type: "number" };
    case "Time":
      return {
        $comment: "originally time",
        type: "object",
        properties: {
          sec: { type: "integer" },
          nsec: { type: "integer" },
        },
      };
    case "Duration":
      return {
        $comment: "originally duration",
        type: "object",
        properties: {
          sec: { type: "integer" },
          nsec: { type: "integer" },
        },
      };
  }
}

export function generateJsonSchema(
  schema: FoxgloveSchema
): Record<string, unknown> {
  const properties: Record<string, unknown> = {};
  for (const field of schema.fields) {
    let fieldType: Record<string, unknown>;
    switch (field.type.type) {
      case "primitive":
        if (field.type.name === "bytes") {
          fieldType = { type: "string", contentEncoding: "base64" };
        } else {
          fieldType = primitiveToJsonSchema(field.type.name); //FIXME
        }
        break;
      case "nested":
        fieldType = generateJsonSchema(field.type.schema);
        break;
      case "enum":
        fieldType = { $comment: "TODO" }; //FIXME;
        break;
    }
    if (field.array === true) {
      fieldType = { type: "array", items: fieldType };
    }
    properties[field.name] = fieldType;
    // if (field.arrayLength != undefined) {
    //   fieldType.$comment = [fieldType.$comment, `length ${field.arrayLength}`]
    //     .filter(Boolean)
    //     .join(", ");
    // }
    // properties[field.name] = fieldType;
  }

  return {
    $comment: `Generated from ${schema.name} by @foxglove/message-schemas`,
    type: "object",
    properties,
  };
}
