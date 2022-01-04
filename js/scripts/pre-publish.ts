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
 * handle the result of spawnSync
 */
function handleResult(step: string, result: any) {
  if (result.status && result.status != 0) {
    if (result.error) {
      throw result.error
    } else {
      throw `Error: ${step} failed`
    }
  }
}

/**
 * build package
 */
function build(loc: string) {
  if (!CommandExists.sync("npm")) {
    throw "npm not installed";
  }

  const buildResult = spawnSync("npm", ["run", "build"], {
    cwd: loc,
    stdio: "inherit",
    env: process.env,
  });
  handleResult("build package", buildResult);
}

/**
 * run program
 */
async function main() {
  const root  = path.resolve(String(await findUp("Cargo.toml")), "..");
  const types = path.resolve(root, "js/types");
  await updateVersion(types);

  build(types);
}

(async () => {
  try {
    await main();
  } catch (e) {
    console.error(e);
    process.exit(1);
  }
})();
