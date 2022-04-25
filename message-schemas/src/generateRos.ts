import { RosMsgField } from "@foxglove/rosmsg";

import { foxgloveSchemas } from ".";
import { FoxglovePrimitive, FoxgloveSchema } from "./types";

type RosMsgFieldWithDescription = RosMsgField & {
  description: string | undefined;
};
type RosMsgDefinitionWithDescription = {
  name: string;
  definitions: RosMsgFieldWithDescription[];
};

function primitiveToRos(type: Exclude<FoxglovePrimitive, "integer" | "bytes">) {
  switch (type) {
    case "string":
      return "string";
    case "boolean":
      return "bool";
    case "float":
      return "float64";
    case "Time":
      return "time";
    case "Duration":
      return "duration";
  }
}

export function generateRosMsgFiles(
  schema: FoxgloveSchema
): Array<{ name: string; filename: string; source: string }> {
  const result: Array<{ name: string; filename: string; source: string }> = [];
  for (const [name, def] of generateRosMsgDefinitions(schema)) {
    let source = "";
    source += `# Generated from ${name} by @foxglove/message-schemas\n`;
    for (const field of def.definitions) {
      if (field.description != undefined) {
        source += `\n# ${field.description}\n`;
      }
      let constant = "";
      if (field.isConstant === true) {
        if (field.valueText == undefined) {
          throw new Error(`Constant ${field.name} has no valueText`);
        }
        constant = `=${field.valueText}`;
      }
      source += `${field.type}${field.isArray === true ? `[]` : ""} ${
        field.name
      }${constant}\n`;
    }
    result.push({ name, filename: `${name}.msg`, source });
  }
  return result;
}

export function generateRosMsgMergedSchema(schema: FoxgloveSchema): string {
  const files = generateRosMsgFiles(schema);
  let result = "";
  for (const { name, source } of files) {
    if (result.length > 0) {
      result += `================================================================================\nMSG: ${name}\n`;
    }
    result += source;
  }
  return result;
}

function* getAllDependencies(
  schema: FoxgloveSchema
): Generator<string, void, void> {
  yield schema.name;
  for (const field of schema.fields) {
    switch (field.type.type) {
      case "enum":
      case "nested":
        yield field.type.name;
        yield* getAllDependencies(
          (foxgloveSchemas as Record<string, FoxgloveSchema>)[field.type.name]!
        );
        break;
      case "primitive":
        break;
    }
  }
}

export function generateRosMsgDefinitions(
  schema: FoxgloveSchema
): Map<string, RosMsgDefinitionWithDescription> {
  const result = new Map<string, RosMsgDefinitionWithDescription>();
  const enumFieldNames = new Set<string>();
  const enumFieldsByEnumName = new Map<string, RosMsgFieldWithDescription[]>();

  const fields: RosMsgFieldWithDescription[] = [];
  for (const field of schema.fields) {
    let isArray = field.array;
    let fieldType: string;
    switch (field.type.type) {
      case "enum": {
        const enumName = field.type.name;
        fieldType = enumName;
        const enumInfo = schema.enums?.find(({ name }) => name === enumName);
        if (!enumInfo) {
          throw new Error(`No enum named ${enumName}`);
        }

        const valueType = "uint8"; // FIXME
        if (enumFieldsByEnumName.has(enumName)) {
          break;
        }
        const enumFields: RosMsgFieldWithDescription[] = [];
        for (const { name, value, description } of enumInfo.values) {
          if (enumFieldNames.has(name)) {
            throw new Error(`Enum value ${name} occurs in more than one enum`);
          }
          enumFieldNames.add(name);
          enumFields.push({
            name,
            value,
            valueText: value.toString(),
            type: valueType,
            description,
          });
        }
        fields.push(...enumFields); //FIXME
        enumFieldsByEnumName.set(enumName, enumFields);
        break;
      }

      case "nested":
        fieldType = field.type.name;
        //FIXME - recurse
        break;

      case "primitive":
        if (field.type.name === "bytes") {
          fieldType = "uint8";
          if (isArray === true) {
            throw new Error("Array of bytes is not supported in ROS msg");
          }
          isArray = true;
        } else if (field.type.name === "integer") {
          fieldType = "int32"; //FIXME
        } else {
          fieldType = primitiveToRos(field.type.name);
        }
        break;
    }
    fields.push({
      name: field.name,
      type: fieldType,
      isComplex: field.type.type === "nested",
      isArray,
      description: field.description,
    });
  }

  result.set(schema.name, { name: schema.name, definitions: fields });

  return result;
}
