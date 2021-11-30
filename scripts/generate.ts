import { RosMsgDefinition } from "@foxglove/rosmsg";
import fs from "fs/promises";
import path from "path";
import { definitions } from "@foxglove/rosmsg-msgs-common";

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

const BUILTIN_TYPE_MAP = new Map([
  ["time", "ros.Time"],
  ["duration", "ros.Duration"],
  ["uint8", "int32"],
  ["uint16", "int32"],
  ["int8", "sint32"],
  ["int16", "sint32"],
  ["float32", "float"],
  ["float64", "double"],
]);

export default async function writeProtobuf(
  outDir: string,
  definitions: Record<string, RosMsgDefinition>
): Promise<void> {
  for (const [typeName, def] of Object.entries(definitions)) {
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
        } else if (BUILTIN_TYPE_MAP.has(field.type)) {
          if (field.type === "time" || field.type === "duration") {
            imports.add("builtins");
          }
          qualifiers.push(BUILTIN_TYPE_MAP.get(field.type)!);
        } else {
          qualifiers.push(field.type);
        }
      }
      let fieldLine = `${qualifiers.join(" ")} ${
        field.name
      } = ${fieldNumber++};`;

      if (field.arrayLength != undefined) {
        fieldLine += ` // [${field.arrayLength}]`;
      }
      fields.push(fieldLine);
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

async function main() {
  const [outDir] = process.argv.slice(2);
  if (!outDir) {
    throw new Error("Missing output directory");
  }

  await writeProtobuf(outDir, definitions);
}

void main();
