/**
 * publish npm packages
 */
import fs from "fs";
import findUp from "find-up";
import path from "path";



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
 * run program
 */
async function main() {
  const root  = path.resolve(String(await findUp("Cargo.toml")), "..");
  const types = path.resolve(root, "js/types");
  await updateVersion(types);
}

(async () => {
  await main();
})();
