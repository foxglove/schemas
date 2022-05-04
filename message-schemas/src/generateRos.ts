import { RosMsgDefinition, RosMsgField } from "@foxglove/rosmsg";
import { definitions as rosCommonDefs } from "@foxglove/rosmsg-msgs-common";

import { FoxgloveMessageSchema, FoxglovePrimitive } from "./types";

type RosMsgFieldWithDescription = RosMsgField & {
  description?: string;
};
type RosMsgDefinitionWithDescription = {
  originalName: string;
  description?: string;
  qualifiedRosName: string;
  fields: RosMsgFieldWithDescription[];
};

function primitiveToRos(
  type: Exclude<FoxglovePrimitive, "integer" | "bytes">,
  { rosVersion }: { rosVersion: 1 | 2 }
) {
  switch (type) {
    case "string":
      return "string";
    case "boolean":
      return "bool";
    case "float":
      return "float64";
    case "Time":
      return rosVersion === 2 ? "builtin_interfaces/Time" : "time";
    case "Duration":
      return rosVersion === 2 ? "builtin_interfaces/Duration" : "duration";
  }
}

export function generateRosMsg(def: RosMsgDefinitionWithDescription): string {
  let source = "";
  source += `# Generated from ${def.originalName} by @foxglove/message-schemas\n`;
  if (def.description != undefined) {
    source += `# ${def.description}\n`;
  }
  for (const field of def.fields) {
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
  return source;
}

type Dependency =
  | { type: "ros"; name: keyof typeof rosCommonDefs }
  | { type: "foxglove"; schema: FoxgloveMessageSchema };

function dependenciesEqual(a: Dependency, b: Dependency) {
  return (
    (a.type === "foxglove" &&
      b.type === "foxglove" &&
      a.schema.name === b.schema.name) ||
    (a.type === "ros" && b.type === "ros" && a.name === b.name)
  );
}

function* getSchemaDependencies(
  schema: FoxgloveMessageSchema
): Iterable<Dependency> {
  for (const field of schema.fields) {
    if (field.type.type === "nested") {
      if (field.type.schema.rosEquivalent != undefined) {
        yield { type: "ros", name: field.type.schema.rosEquivalent };
        yield* getRosDependencies(
          rosCommonDefs[field.type.schema.rosEquivalent]
        );
      } else {
        yield { type: "foxglove", schema: field.type.schema };
        yield* getSchemaDependencies(field.type.schema);
      }
    }
  }
}
function* getRosDependencies(schema: RosMsgDefinition): Iterable<Dependency> {
  for (const field of schema.definitions) {
    if (field.isComplex === true) {
      yield { type: "ros", name: field.type as keyof typeof rosCommonDefs };
      yield* getRosDependencies(
        rosCommonDefs[field.type as keyof typeof rosCommonDefs]!
      );
    }
  }
}

//FIXME: what to do with enums?
export function generateRosMsgDefinition(
  schema: FoxgloveMessageSchema,
  { rosVersion }: { rosVersion: 1 | 2 }
): RosMsgDefinitionWithDescription {
  // const result: RosMsgDefinitionWithDescription[] = [];
  const enumFieldNames = new Set<string>();
  const seenEnumNames = new Set<string>();

  const fields: RosMsgFieldWithDescription[] = [];
  for (const field of schema.fields) {
    let isArray = field.array;
    let fieldType: string;
    switch (field.type.type) {
      case "enum": {
        // Add enum constants preceding the field so that Studio can pick them up:
        // https://foxglove.dev/docs/studio/annotating-data
        const enumName = field.type.enum.name;
        const valueType = "uint8";
        fieldType = valueType;
        if (seenEnumNames.has(enumName)) {
          break;
        }
        const enumFields: RosMsgFieldWithDescription[] = [];
        for (const { name, value, description } of field.type.enum.values) {
          if (enumFieldNames.has(name)) {
            throw new Error(
              `Enum value ${name} occurs in more than one enum referenced by ${schema.name}, this is not supported in ROS msg files`
            );
          }
          if (value < 0 || value > 255 || !Number.isInteger(value)) {
            throw new Error(
              `Only uint8 enums are currently supported; value ${name}=${value} is out of range`
            );
          }
          enumFieldNames.add(name);
          enumFields.push({
            name,
            value,
            isConstant: true,
            valueText: value.toString(),
            type: valueType,
            description,
          });
        }
        fields.push(...enumFields);
        seenEnumNames.add(enumName);
        break;
      }

      case "nested":
        if (field.type.schema.rosEquivalent != undefined) {
          fieldType = field.type.schema.rosEquivalent;
        } else {
          fieldType = `foxglove_msgs/${field.type.schema.name}`;
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
          fieldType = primitiveToRos(field.type.name, { rosVersion });
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

  return {
    originalName: schema.name,
    qualifiedRosName: `foxglove_msgs/${schema.name}`,
    fields,
  };
}

export function generateRosMsgMergedSchema(
  schema: FoxgloveMessageSchema,
  { rosVersion }: { rosVersion: 1 | 2 }
): string {
  const dependencies: Dependency[] = [];
  for (const dep of getSchemaDependencies(schema)) {
    if (!dependencies.some((existing) => dependenciesEqual(existing, dep))) {
      dependencies.push(dep);
    }
  }

  let result = generateRosMsg(generateRosMsgDefinition(schema, { rosVersion }));
  for (const dep of dependencies) {
    let name: string;
    let source: string;
    if (dep.type === "ros") {
      name = dep.name;
      source = generateRosMsg({
        originalName: dep.name,
        qualifiedRosName: dep.name,
        fields: rosCommonDefs[dep.name].definitions,
      });
    } else {
      const definition = generateRosMsgDefinition(dep.schema, { rosVersion });
      name = definition.qualifiedRosName;
      source = generateRosMsg(definition);
    }
    result += `================================================================================\nMSG: ${name}\n${source}`;
  }
  return result;
}
