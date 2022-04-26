import { program } from "commander";
import fs from "fs/promises";
import path from "path";
import rimraf from "rimraf";
import { promisify } from "util";

import { generateJsonSchema } from "../src/generateJsonSchema";
import { generateProto } from "../src/generateProto";
import { foxgloveSchemas } from "../src/schemas";

async function main({ outDir }: { outDir: string }) {
  await promisify(rimraf)(outDir);

  for (const schema of Object.values(foxgloveSchemas)) {
    await fs.mkdir(path.join(outDir, "json"), { recursive: true });
    await fs.writeFile(
      path.join(outDir, "json", `${schema.name}.json`),
      JSON.stringify(generateJsonSchema(schema), undefined, 2)
    );

    await fs.mkdir(path.join(outDir, "proto", "foxglove"), { recursive: true });
    await fs.writeFile(
      path.join(outDir, "proto", "foxglove", `${schema.name}.proto`),
      generateProto(schema)
    );
  }
}

program.requiredOption("-o, --out-dir <dir>", "output directory").action(main);

program.parseAsync().catch(console.error);
