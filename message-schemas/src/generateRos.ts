import { RosMsgDefinition, RosMsgField } from "@foxglove/rosmsg";
import { definitions as rosCommonDefs } from "@foxglove/rosmsg-msgs-common";

import { FoxglovePrimitive, FoxgloveSchema } from "./types";

type RosMsgFieldWithDescription = RosMsgField & {
  description?: string;
};
type RosMsgDefinitionWithDescription = {
  originalName: string;
  qualifiedRosName: string;
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
  for (const def of generateRosMsgDefinitions(schema)) {
    let source = "";
    source += `# Generated from ${def.originalName} by @foxglove/message-schemas\n`;
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
    result.push({
      name: def.qualifiedRosName,
      filename: `${def.qualifiedRosName}.msg`,
      source,
    });
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

export function generateRosMsgDefinitions(
  rootSchema: FoxgloveSchema
): RosMsgDefinitionWithDescription[] {
  const seenTypes = new Set<string>();
  const result: RosMsgDefinitionWithDescription[] = [];
  const enumFieldNames = new Set<string>();
  const enumFieldsByEnumName = new Map<string, RosMsgFieldWithDescription[]>();

  function addRosMsgDefinition(def: RosMsgDefinition) {
    if (def.name == undefined) {
      throw new Error("Cannot add definition with no name");
    }
    result.unshift({
      ...def,
      originalName: def.name,
      qualifiedRosName: def.name,
    });
    seenTypes.add(def.name);
    for (const field of def.definitions) {
      if (field.isComplex === true) {
        if (field.type in rosCommonDefs && !seenTypes.has(field.type)) {
          addRosMsgDefinition(
            rosCommonDefs[field.type as keyof typeof rosCommonDefs]
          );
        }
      }
    }
  }

  function addSchema(schema: FoxgloveSchema) {
    const fields: RosMsgFieldWithDescription[] = [];
    for (const field of schema.fields) {
      let isArray = field.array;
      let fieldType: string;
      switch (field.type.type) {
        case "enum": {
          const enumName = field.type.enum.name;
          fieldType = enumName;

          const valueType = "uint8"; // FIXME
          if (enumFieldsByEnumName.has(enumName)) {
            break;
          }
          const enumFields: RosMsgFieldWithDescription[] = [];
          for (const { name, value, description } of field.type.enum.values) {
            if (enumFieldNames.has(name)) {
              throw new Error(
                `Enum value ${name} occurs in more than one enum`
              );
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
          fields.push(...enumFields); //FIXME do we need to store whole array in map?
          enumFieldsByEnumName.set(enumName, enumFields);
          break;
        }

        case "nested":
          if (field.type.schema.rosEquivalent != undefined) {
            fieldType = field.type.schema.rosEquivalent;
            addRosMsgDefinition(rosCommonDefs[field.type.schema.rosEquivalent]);
          } else {
            fieldType = field.type.schema.name;
            addSchema(field.type.schema);
          }
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

    result.unshift({
      originalName: schema.name,
      qualifiedRosName: `foxglove_msgs/${schema.name}`,
      definitions: fields,
    });
    seenTypes.add(schema.name);
  }

  addSchema(rootSchema);

  return result;
}
