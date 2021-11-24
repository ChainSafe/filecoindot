/**
 * e2e runner
 */
import Api from "./api";
import Launch from "./launch";
import { ChildProcess } from "child_process";

const OCW = "offchain-worker";
const OCW_PREPARED = "haven't set filecoin rpc yet";

// Kill subprocesses
function killAll(ps: ChildProcess, exitCode: number) {
  try {
    if (ps.send && !ps.killed) {
      ps.send("exit");
    }
    ps.kill("SIGINT");
  } catch (e) {
    if (e.code !== "EPERM") {
      process.stdout.write(e);
      process.exit(2);
    }
  }

  process.exit(exitCode);
}

/**
 * e2e runner config
 */
export interface RunnerConfig {
  address: string;
  filecoindotRpc: string;
  id: string;
  suri: string;
  ws: string;
}

/**
 * e2e runner
 */
export default class Runner {
  config: RunnerConfig;

  constructor(config: RunnerConfig) {
    this.config = config;
  }

  public async run() {
    const { ws, filecoindotRpc, id, suri, address } = this.config;

    // start launching
    console.log("bootsrap filecoindot template...");
    const ps = await Launch.launch("pipe");
    if (ps.stdout) {
      ps.stdout.on("data", async (chunk: Buffer) => {
        process.stdout.write(chunk.toString());
      });
    }

    // start testing
    let started: boolean = false;
    if (ps.stderr) {
      ps.stderr.on("data", async (chunk: Buffer) => {
        chunk.includes(OCW) && process.stderr.write(chunk.toString());
        if (chunk.includes(OCW_PREPARED)) {
          if (!started) {
            started = true;
            const api = await Api.New(ws);
            await api.insertAuthor(id, suri);
            await api.setEndpoint(filecoindotRpc);
            await api.addRelayer(address);
          }
        }
      });
    }

    // kill all processes when exiting.
    process.on("exit", () => {
      killAll(ps, Number(process.exitCode));
    });

    // handle ctrl+c to trigger `exit`.
    process.on("SIGINT", () => killAll(ps, 0));
  }
}
