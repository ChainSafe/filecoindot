/**
 * publish npm packages
 */
import fs from "fs";
import findUp from "find-up";
import path from "path";
import CommandExists from "command-exists"
import { spawnSync } from "child_process"



/**
 * update minor version
 */
async function updateVersion(loc: string): Promise<void> {
  const pkgJson: string = path.resolve(loc, "package.json");
  let pkg: any = (await import(pkgJson)).default;
  const version: string = pkg.version.replace(/\d+$/, (v: string) => {
    return String(Number.parseInt(v) + 1);
  });

  // reset version
  pkg.version = version;
  fs.writeFileSync(pkgJson, JSON.stringify(pkg, null, 2));
}

/**
 * build and publish package
 */
function buildAndPublish(loc: string) {
  if (!CommandExists.sync("npm")) {
    throw "npm not installed";
  }

  return spawnSync("npm", ["run", "publish"], {
    cwd: loc,
    stdio: "inherit"
  });
}

/**
 * run program
 */
async function main() {
  const root  = path.resolve(String(await findUp("Cargo.toml")), "..");
  const types = path.resolve(root, "js/types");
  await updateVersion(types);

  const result = buildAndPublish(types);
  if (result.status != 0) {
    throw "Error: publish package failed";
  }
}

(async () => {
  try {
    await main();
  } catch (e) {
    console.error(e);
  }
})();
