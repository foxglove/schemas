import { program } from "commander";
import fs from "fs/promises";
import path from "path";
import rimraf from "rimraf";
import { promisify } from "util";

import { generateRosMsg, generateRosMsgDefinition } from "../internal";
import {
  BYTE_VECTOR_FB,
  DURATION_FB,
  generateFlatbuffers,
  TIME_FB,
} from "../internal/generateFlatbuffers";
import { generateJsonSchema } from "../internal/generateJsonSchema";
import { generateMarkdown } from "../internal/generateMarkdown";
import { generateProto } from "../internal/generateProto";
import { generateTypeScript, DURATION_TS, TIME_TS } from "../internal/generateTypeScript";
import { foxgloveEnumSchemas, foxgloveMessageSchemas } from "../internal/schemas";

async function logProgress(message: string, body: () => Promise<void>) {
  process.stderr.write(`${message}... `);
  await body();
  process.stderr.write("done\n");
}

async function main({ outDir, rosOutDir }: { outDir: string; rosOutDir: string }) {
  await logProgress("Removing any existing output directory", async () => {
    await promisify(rimraf)(outDir);
  });

  await logProgress("Generating JSONSchema definitions", async () => {
    await fs.mkdir(path.join(outDir, "jsonschema"), { recursive: true });
    let indexTS = "// Generated by https://github.com/foxglove/schemas\n\n";
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      const json = JSON.stringify(generateJsonSchema(schema), undefined, 2);
      await fs.writeFile(path.join(outDir, "jsonschema", `${schema.name}.json`), json + "\n");
      indexTS += `export const ${schema.name} = ${json};\n\n`;
    }
    await fs.writeFile(path.join(outDir, "jsonschema", `index.ts`), indexTS);
  });

  await logProgress("Generating ROS 1 msg files", async () => {
    await fs.mkdir(path.join(outDir, "ros1"), { recursive: true });
    await fs.mkdir(path.join(rosOutDir, "ros1"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      if (schema.rosEquivalent != undefined) {
        continue;
      }
      const msg = generateRosMsg(generateRosMsgDefinition(schema, { rosVersion: 1 }));
      await fs.writeFile(path.join(outDir, "ros1", `${schema.name}.msg`), msg);
      await fs.writeFile(path.join(rosOutDir, "ros1", `${schema.name}.msg`), msg);
    }
  });

  await logProgress("Generating ROS 2 msg files", async () => {
    await fs.mkdir(path.join(outDir, "ros2"), { recursive: true });
    await fs.mkdir(path.join(rosOutDir, "ros2"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      if (schema.rosEquivalent != undefined) {
        continue;
      }
      const msg = generateRosMsg(generateRosMsgDefinition(schema, { rosVersion: 2 }));
      await fs.writeFile(path.join(outDir, "ros2", `${schema.name}.msg`), msg);
      await fs.writeFile(path.join(rosOutDir, "ros2", `${schema.name}.msg`), msg);
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

  await logProgress("Generating FlatBuffer definitions", async () => {
    await fs.mkdir(path.join(outDir, "flatbuffers"), { recursive: true });
    await fs.writeFile(path.join(outDir, "flatbuffers", "ByteVector.fbs"), BYTE_VECTOR_FB);
    await fs.writeFile(path.join(outDir, "flatbuffers", "Time.fbs"), TIME_FB);
    await fs.writeFile(path.join(outDir, "flatbuffers", "Duration.fbs"), DURATION_FB);
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      await fs.writeFile(
        path.join(outDir, "flatbuffers", `${schema.name}.fbs`),
        generateFlatbuffers(schema),
      );
    }
    for (const schema of Object.values(foxgloveEnumSchemas)) {
      await fs.writeFile(
        path.join(outDir, "flatbuffers", `${schema.name}.fbs`),
        generateFlatbuffers(schema),
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

  await logProgress("Generating README.md", async () => {
    await fs.writeFile(
      path.join(outDir, "README.md"),
      generateMarkdown(Object.values(foxgloveMessageSchemas), Object.values(foxgloveEnumSchemas)),
    );
  });
}

program
  .requiredOption("-o, --out-dir <dir>", "output directory")
  .requiredOption("--ros-out-dir <dir>", "output directory for additional copies of ROS msgs")
  .action(main);

program.parseAsync().catch(console.error);
