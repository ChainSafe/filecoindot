/**
 * bootstrap filecoindot-template binary
 */
import { ChildProcess, StdioOptions, spawn, spawnSync } from "child_process";
import fs from "fs";
import { findUpSync } from "find-up";
import path from "path";

export async function launch(stdio?: StdioOptions): Promise<ChildProcess> {
  const root = path.resolve(String(findUpSync("Cargo.toml")), "..");
  const bin = path.resolve(String(root), "target/release/filecoindot-template");

  // Build binary if not exist
  if (!fs.existsSync(bin)) {
    spawnSync("cargo", ["build", "--release"], {
      cwd: root,
      stdio,
    });
  }

  // spawn `fileconidot-template`
  return spawn(bin, ["--dev", "--tmp"], { stdio });
}
