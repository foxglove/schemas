import fs from "fs/promises";
import path from "path";
import { definitions as allMsgDefinitions } from "@foxglove/rosmsg-msgs-common";
import { program } from "commander";
import { RosMsgDefinition } from "@foxglove/rosmsg";

const BUILTINS_PROTO = `syntax = "proto3";

package ros;

message Time {
  fixed32 sec = 1;
  fixed32 nsec = 2;
}

message Duration {
  fixed32 sec = 1;
  fixed32 nsec = 2;
}
`;
const BUILTIN_JSON_TYPE_MAP = new Map<string, Record<string, unknown>>([
  [
    "time",
    {
      $comment: "originally time",
      sec: { type: "integer" },
      nsec: { type: "integer" },
    },
  ],
  [
    "duration",
    {
      $comment: "originally duration",
      sec: { type: "integer" },
      nsec: { type: "integer" },
    },
  ],
  ["uint8", { $comment: "originally uint8", type: "integer" }],
  ["uint16", { $comment: "originally uint16", type: "integer" }],
  ["uint32", { $comment: "originally uint32", type: "integer" }],
  ["uint64", { $comment: "originally uint64", type: "integer" }],
  ["int8", { $comment: "originally int8", type: "integer" }],
  ["int16", { $comment: "originally int16", type: "integer" }],
  ["int32", { $comment: "originally int32", type: "integer" }],
  ["int64", { $comment: "originally int64", type: "integer" }],
  ["float32", { $comment: "originally float32", type: "number" }],
  ["float64", { $comment: "originally float64", type: "number" }],
  ["string", { type: "string" }],
  ["bool", { type: "boolean" }],
]);

const BUILTIN_PROTO_TYPE_MAP = new Map([
  ["time", "ros.Time"],
  ["duration", "ros.Duration"],
  ["uint8", "int32"],
  ["uint16", "int32"],
  ["int8", "sint32"],
  ["int16", "sint32"],
  ["float32", "float"],
  ["float64", "double"],
]);

async function writeProtobuf({ outDir }: { outDir: string }): Promise<void> {
  for (const [typeName, def] of Object.entries(allMsgDefinitions)) {
    const nameParts = typeName.split("/");
    if (nameParts.length !== 2) {
      throw new Error(`Invalid name ${typeName}`);
    }
    const packageName = nameParts[0]!;
    const msgName = nameParts[1]!;

    const imports = new Set<string>();

    const fields: string[] = [];
    let fieldNumber = 1;
    for (const field of def.definitions) {
      if (field.isConstant === true) {
        // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
        fields.push(
          `// ${field.type} ${field.name} = ${
            field.valueText ?? field.value ?? ""
          }`
        );
        continue;
      }
      const lineComments: string[] = [];
      const qualifiers = [];
      if (field.type === "uint8" && field.isArray === true) {
        qualifiers.push("bytes");
      } else {
        if (field.isArray === true) {
          qualifiers.push("repeated");
        }
        if (field.isComplex === true) {
          qualifiers.push(`ros.${field.type.replace("/", ".")}`);
          imports.add(field.type);
        } else if (BUILTIN_PROTO_TYPE_MAP.has(field.type)) {
          if (field.type === "time" || field.type === "duration") {
            imports.add("builtins");
          }
          const protoType = BUILTIN_PROTO_TYPE_MAP.get(field.type)!;
          if (protoType.includes("int")) {
            lineComments.push(`originally ${field.type}`);
          }
          qualifiers.push(BUILTIN_PROTO_TYPE_MAP.get(field.type)!);
        } else {
          qualifiers.push(field.type);
        }
      }
      if (field.arrayLength != undefined) {
        lineComments.push(`length ${field.arrayLength}`);
      }
      fields.push(
        `${qualifiers.join(" ")} ${field.name} = ${fieldNumber++};${
          lineComments.length > 0 ? " // " + lineComments.join(", ") : ""
        }`
      );
    }

    const outputSections = [
      `// Generated from ${typeName}.msg`,

      'syntax = "proto3";',

      Array.from(imports)
        .sort()
        .map((name) => `import "ros/${name}.proto";`)
        .join("\n"),

      `package ros.${packageName};`,

      `message ${msgName} {\n  ${fields.join("\n  ")}\n}`,
    ];

    const packageDir = path.join(outDir, "ros", packageName);
    await fs.mkdir(packageDir, { recursive: true });
    await fs.writeFile(
      path.join(packageDir, `${msgName}.proto`),
      outputSections.filter(Boolean).join("\n\n") + "\n"
    );
  }

  await fs.mkdir(path.join(outDir, "ros"), { recursive: true });
  await fs.writeFile(
    path.join(outDir, "ros", "builtins.proto"),
    BUILTINS_PROTO
  );
}

async function writeJsonSchema({ outDir }: { outDir: string }): Promise<void> {
  function rosMsgDefinitionToJsonSchema(
    typeName: string,
    def: RosMsgDefinition
  ): Record<string, unknown> {
    const properties: Record<string, unknown> = {};

    for (const field of def.definitions) {
      if (field.isConstant === true) {
        continue;
      }
      let fieldType: Record<string, unknown>;
      if (field.type === "uint8" && field.isArray === true) {
        fieldType = { type: "string", contentEncoding: "base64" };
      } else {
        if (field.isComplex === true) {
          fieldType = rosMsgDefinitionToJsonSchema(
            field.type,
            allMsgDefinitions[field.type as keyof typeof allMsgDefinitions]!
          );
        } else if (BUILTIN_JSON_TYPE_MAP.has(field.type)) {
          fieldType = BUILTIN_JSON_TYPE_MAP.get(field.type)!;
        } else {
          throw new Error("unsupported field type " + field.type);
        }
        if (field.isArray === true) {
          fieldType = { type: "array", items: fieldType };
        }
        properties[field.name] = fieldType;
      }
      if (field.arrayLength != undefined) {
        fieldType.$comment = [fieldType.$comment, `length ${field.arrayLength}`]
          .filter(Boolean)
          .join(", ");
      }
      properties[field.name] = fieldType;
    }

    return {
      $comment: `Generated from ${typeName}.msg`,
      type: "object",
      properties,
    };
  }

  for (const [typeName, def] of Object.entries(allMsgDefinitions)) {
    const nameParts = typeName.split("/");
    if (nameParts.length !== 2) {
      throw new Error(`Invalid name ${typeName}`);
    }
    const packageName = nameParts[0]!;
    const msgName = nameParts[1]!;

    const jsonSchema = rosMsgDefinitionToJsonSchema(typeName, def);

    const packageDir = path.join(outDir, "ros", packageName);
    await fs.mkdir(packageDir, { recursive: true });
    await fs.writeFile(
      path.join(packageDir, `${msgName}.json`),
      JSON.stringify(jsonSchema, undefined, 2) + "\n"
    );
  }

  await fs.mkdir(path.join(outDir, "ros"), { recursive: true });
}

program
  .command("proto")
  .requiredOption("-o, --out-dir <dir>", "output directory")
  .action(writeProtobuf);

program
  .command("json")
  .requiredOption("-o, --out-dir <dir>", "output directory")
  .action(writeJsonSchema);

program.parseAsync().catch(console.error);
