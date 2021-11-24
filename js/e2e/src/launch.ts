/**
 * bootstrap filecoindot-template binary
 */
import { ChildProcess, StdioOptions, spawn, spawnSync } from "child_process";
import fs from "fs";
import findUp from "find-up";
import path from "path";

async function launch(stdio?: StdioOptions): Promise<ChildProcess> {
  const root = path.resolve(String(await findUp("Cargo.toml")), "..");
  const bin = path.resolve(root, "target/release/filecoindot-template");

  // Build binary if not exist
  if (!fs.existsSync(bin)) {
    spawnSync("cargo", ["build", "--release"], {
      cwd: root,
      stdio,
    });
  }

  // spawn `fileconidot-template`
  return spawn(bin, ["--dev", "--tmp"], {
    stdio,
    env: {
      RUST_LOG: "runtime",
    },
  });
}

export default {
  launch,
};
