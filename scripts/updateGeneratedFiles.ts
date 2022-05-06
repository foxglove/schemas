import { program } from "commander";
import fs from "fs/promises";
import path from "path";
import rimraf from "rimraf";
import { promisify } from "util";

import { generateRosMsg, generateRosMsgDefinition } from "../src";
import { generateJsonSchema } from "../src/generateJsonSchema";
import { generateProto } from "../src/generateProto";
import { generateTypeScript, DURATION_TS, TIME_TS } from "../src/generateTypeScript";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "../src/schemas";

async function logProgress(message: string, body: () => Promise<void>) {
  process.stderr.write(`${message}... `);
  await body();
  process.stderr.write("done\n");
}

async function main({ outDir }: { outDir: string }) {
  await logProgress("Removing any existing output directory", async () => {
    await promisify(rimraf)(outDir);
  });

  await logProgress("Generating JSONSchema definitions", async () => {
    await fs.mkdir(path.join(outDir, "jsonschema"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      await fs.writeFile(
        path.join(outDir, "jsonschema", `${schema.name}.json`),
        JSON.stringify(generateJsonSchema(schema), undefined, 2) + "\n",
      );
    }
  });

  await logProgress("Generating ROS 1 msg files", async () => {
    await fs.mkdir(path.join(outDir, "ros1"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      if (schema.rosEquivalent != undefined) {
        continue;
      }
      await fs.writeFile(
        path.join(outDir, "ros1", `${schema.name}.msg`),
        generateRosMsg(generateRosMsgDefinition(schema, { rosVersion: 1 })),
      );
    }
  });

  await logProgress("Generating ROS 2 msg files", async () => {
    await fs.mkdir(path.join(outDir, "ros2"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      if (schema.rosEquivalent != undefined) {
        continue;
      }
      await fs.writeFile(
        path.join(outDir, "ros2", `${schema.name}.msg`),
        generateRosMsg(generateRosMsgDefinition(schema, { rosVersion: 2 })),
      );
    }
  });

  await logProgress("Generating Protobuf definitions", async () => {
    await fs.mkdir(path.join(outDir, "proto", "foxglove"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      const enums = Object.values(foxgloveEnumSchemas).filter(
        (enumSchema) => enumSchema.protobufParentMessageName === schema.name,
      );
      await fs.writeFile(
        path.join(outDir, "proto", "foxglove", `${schema.name}.proto`),
        generateProto(schema, enums),
      );
    }
  });

  await logProgress("Generating TypeScript definitions", async () => {
    await fs.mkdir(path.join(outDir, "typescript"), { recursive: true });
    await fs.writeFile(path.join(outDir, "typescript", "Time.ts"), TIME_TS);
    await fs.writeFile(path.join(outDir, "typescript", "Duration.ts"), DURATION_TS);
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      await fs.writeFile(
        path.join(outDir, "typescript", `${schema.name}.ts`),
        generateTypeScript(schema),
      );
    }
    for (const schema of Object.values(foxgloveEnumSchemas)) {
      await fs.writeFile(
        path.join(outDir, "typescript", `${schema.name}.ts`),
        generateTypeScript(schema),
      );
    }
    const allSchemaNames = [
      ...Object.values(foxgloveMessageSchemas),
      ...Object.values(foxgloveEnumSchemas),
    ].sort((a, b) => a.name.localeCompare(b.name));
    let indexTS = "";
    for (const schema of allSchemaNames) {
      indexTS += `export * from "./${schema.name}";\n`;
    }
    await fs.writeFile(path.join(outDir, "typescript", `index.ts`), indexTS);
  });
}

program.requiredOption("-o, --out-dir <dir>", "output directory").action(main);

program.parseAsync().catch(console.error);
