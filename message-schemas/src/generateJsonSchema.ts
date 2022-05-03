import { FoxgloveMessageSchema, FoxglovePrimitive } from "./types";

function primitiveToJsonSchema(type: Exclude<FoxglovePrimitive, "bytes">) {
  //FIXME
  switch (type) {
    case "string":
      return { type: "string" };
    case "boolean":
      return { type: "boolean" };
    case "float":
      return { type: "number" };
    case "integer":
      return { type: "integer" };
    case "Time":
      return {
        type: "object",
        title: "Time",
        properties: {
          sec: { type: "integer" },
          nsec: { type: "integer" },
        },
      };
    case "Duration":
      return {
        type: "object",
        title: "Duration",
        properties: {
          sec: { type: "integer" },
          nsec: { type: "integer" },
        },
      };
  }
}

export function generateJsonSchema(
  schema: FoxgloveMessageSchema
): Record<string, unknown> {
  const properties: Record<string, unknown> = {};
  for (const field of schema.fields) {
    let fieldType: Record<string, unknown>;
    switch (field.type.type) {
      case "primitive":
        if (field.type.name === "bytes") {
          fieldType = {
            type: "string",
            description: field.description,
            contentEncoding: "base64",
          };
        } else {
          fieldType = {
            description: field.description,
            ...primitiveToJsonSchema(field.type.name),
          }; //FIXME
        }
        break;
      case "nested":
        fieldType = {
          description: field.description,
          ...generateJsonSchema(field.type.schema),
        };
        //FIXME: TODO required?
        break;
      case "enum":
        fieldType = {
          title: `${field.type.enum.name}: ${field.type.enum.description}`,
          description: field.description,
          oneOf: field.type.enum.values.map(({ name, value, description }) => ({
            title: name,
            const: value,
            description,
          })),
        };
        break;
    }
    if (field.array === true) {
      const { description, ...rest } = fieldType;
      fieldType = { type: "array", description, ...rest };
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
    title: schema.name,
    description: schema.description,
    type: "object",
    properties,
  };
}
