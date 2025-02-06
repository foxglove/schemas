import { spawn } from "child_process";
import fs from "fs/promises";
import { glob } from "glob";
import path from "path";

const versionRegex = /^version\s*=\s*"[^"]*"/m;

async function main() {
  const newVersion = process.argv[2];
  if (newVersion?.startsWith("v") !== true) {
    console.log("Usage: bumpSdkVersion.ts <version>");
    console.log("Version must start with 'v'");
    process.exit(1);
  }

  // Remove the 'v' prefix for the actual version string
  const versionNumber = newVersion.slice(1);

  // Find all Cargo.toml files from workspace root
  const workspaceRoot = path.resolve(__dirname, "..");
  const cargoFiles = await glob("**/Cargo.toml", {
    ignore: ["**/target/**", "**/node_modules/**"],
    cwd: workspaceRoot,
    absolute: true,
  });

  let success = true;
  for (const cargoFile of cargoFiles) {
    console.log(`Checking ${cargoFile}...`);
    const content = await fs.readFile(cargoFile, "utf8");

    if (!versionRegex.test(content)) {
      console.log(`  ℹ️ Skipped, does not contain version field`);
      continue;
    }

    // Only update the main version field, not dependencies
    const updatedContent = content.replace(versionRegex, `version = "${versionNumber}"`);

    if (content === updatedContent) {
      const oldVersion = versionRegex.exec(content)?.[0] ?? '""';
      console.error(`  ❌ Version could not be updated from ${oldVersion} to "${versionNumber}"`);
      success = false;
    } else {
      await fs.writeFile(cargoFile, updatedContent);
      console.log(`  ✅ Updated version in ${cargoFile} to ${versionNumber}`);
    }
  }

  if (!success) {
    console.error("\n❌ Some versions could not be updated");
    process.exit(1);
  }

  // run cargo check
  console.log("\nRunning cargo check...");
  const cargoCheck = spawn("cargo", ["check"], {
    cwd: workspaceRoot,
    stdio: "inherit",
  });

  const exitCode = await new Promise<number>((resolve) => {
    cargoCheck.on("close", resolve);
  });

  if (exitCode !== 0) {
    process.exit(exitCode);
  }
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
